use vk::{self, builders::*, types::*};

use fnd::{
    alloc::Allocator,
    containers::{Array, String},
    dl::DynamicLibrary,
    str::CStr,
    Shared,
};

use core::{ffi::c_void, mem::transmute};

use crate::hal;

impl From<VkPhysicalDeviceType> for hal::GpuType
{
    fn from(vk_type: VkPhysicalDeviceType) -> hal::GpuType
    {
        match vk_type
        {
            VkPhysicalDeviceType::CPU => hal::GpuType::Cpu,
            VkPhysicalDeviceType::DISCRETE_GPU => hal::GpuType::DiscreteGpu,
            VkPhysicalDeviceType::INTEGRATED_GPU => hal::GpuType::IntegratedGpu,
            VkPhysicalDeviceType::VIRTUAL_GPU => hal::GpuType::VirtualGpu,
            _ => hal::GpuType::Unknown,
        }
    }
}

impl From<VkQueueFlags> for hal::QueueType
{
    fn from(vk_flags: VkQueueFlags) -> hal::QueueType
    {
        let has_graphics = vk_flags.contains(VkQueueFlags::GRAPHICS_BIT);
        let has_compute = vk_flags.contains(VkQueueFlags::COMPUTE_BIT);
        let has_transfer = vk_flags.contains(VkQueueFlags::TRANSFER_BIT);

        match (has_graphics, has_compute, has_transfer)
        {
            (true, true, _) => hal::QueueType::General,
            (true, false, _) => hal::QueueType::Graphics,
            (false, true, _) => hal::QueueType::Compute,
            (false, false, true) => hal::QueueType::Transfer,
            _ => unreachable!(),
        }
    }
}

pub struct Backend;

impl hal::Backend for Backend
{
    type Error = Error;
    type Instance = Instance;
    type PhysicalDevice = VkPhysicalDevice;
    type QueueFamily = QueueFamily;
    type Device = Device;
}

pub struct RawInstance
{
    dl: DynamicLibrary,
    entry: vk::EntryPoint,
    instance: vk::Instance,
    debug_utils_messenger: Option<VkDebugUtilsMessengerEXT>,
}

impl Drop for RawInstance
{
    fn drop(&mut self)
    {
        if let Some(debug_utils_messenger) = self.debug_utils_messenger
        {
            self.instance
                .destroy_debug_utils_messenger_ext(debug_utils_messenger, None);
        }

        self.instance.destroy_instance(None);
    }
}

#[derive(Debug)]
pub enum Error
{
    Instance(InstanceError),
    VulkanError(VkResult),
}

extern "system" fn messenger_cb(
    _message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: VkDebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut core::ffi::c_void,
) -> VkBool32
{
    unsafe {
        let callback_data: &VkDebugUtilsMessengerCallbackDataEXT = &*p_callback_data;
        match CStr::from_bytes_null_terminated_unchecked(callback_data.p_message).as_str()
        {
            Ok(s) => println!("{}", s),
            _ =>
            {}
        }
    }

    return VK_FALSE;
}

#[derive(Debug)]
pub enum InstanceError
{
    CannotLoadLibrary,
    CreationError(VkResult),
    DeviceCreationError(VkResult),
    InvalidPhysicalDeviceName,
}

pub struct Instance
{
    raw: Shared<RawInstance>,
}

impl Instance
{
    fn build_debug(vk_instance: &vk::Instance) -> Result<VkDebugUtilsMessengerEXT, Error>
    {
        let create_info = VkDebugUtilsMessengerCreateInfoEXTBuilder::new()
            .message_severity(
                VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR_BIT_EXT
                    | VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING_BIT_EXT,
            )
            .message_type(
                VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL_BIT_EXT
                    | VkDebugUtilsMessageTypeFlagBitsEXT::VALIDATION_BIT_EXT,
            )
            .pfn_user_callback(Some(messenger_cb));

        vk_instance
            .create_debug_utils_messenger_ext(&create_info, None)
            .map(|(_, handle)| handle)
            .map_err(|e| Error::VulkanError(e))
    }

    pub fn create() -> Result<Self, Error>
    {
        let dl = DynamicLibrary::load("vulkan-1.dll")
            .ok_or(Error::Instance(InstanceError::CannotLoadLibrary))?;

        let vk_entry = vk::EntryPoint::new(|name_slice| unsafe {
            dl.get_symbol_from_bytes_null_terminated(name_slice)
                .map(|f: *mut c_void| transmute(f))
        });

        // @Todo Use SmallVec or something similar like a small allocator.
        let mut layers = Array::new();
        let mut extensions = Array::new();

        if cfg!(debug_assertions)
        {
            extensions.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr());
            layers.push(b"VK_LAYER_KHRONOS_validation\0".as_ptr());
        }

        let app_info = VkApplicationInfoBuilder::new();
        let create_info = VkInstanceCreateInfoBuilder::new()
            .p_application_info(Some(&app_info))
            .pp_enabled_extension_names(&extensions)
            .pp_enabled_layer_names(&layers);

        let vk_instance = vk_entry
            .create_instance(&create_info, None)
            .map(|(_, instance)| instance)
            .map_err(|vk_result| Error::Instance(InstanceError::CreationError(vk_result)))?;

        let vk_instance = vk::Instance::new(vk_instance, &vk_entry);

        let debug_utils_messenger = if cfg!(debug_assertions)
        {
            Some(Self::build_debug(&vk_instance)?)
        }
        else
        {
            None
        };

        Ok(Self {
            raw: Shared::new(RawInstance {
                dl,
                entry: vk_entry,
                instance: vk_instance,
                debug_utils_messenger,
            }),
        })
    }
}

impl hal::Instance<Backend> for Instance
{
    fn enumerate_gpus_with<A: Allocator + Clone>(
        &self,
        a: A,
    ) -> Result<Array<hal::Gpu<Backend, A>, A>, Error>
    {
        let gpu_count = self
            .raw
            .instance
            .enumerate_physical_devices_count()
            .map_err(|e| Error::VulkanError(e))?
            .1;

        let mut gpus = Array::new();
        gpus.resize_default(gpu_count);

        self.raw
            .instance
            .enumerate_physical_devices(&mut gpus)
            .map_err(|e| Error::VulkanError(e))?;

        let mut result_gpus = Array::new_with(a.clone());
        result_gpus.reserve(gpu_count);

        for gpu in gpus
        {
            let prps = self.raw.instance.get_physical_device_properties(gpu);
            let c_name =
                unsafe { CStr::from_bytes_null_terminated_unchecked(prps.device_name.as_ptr()) };
            let name = String::from_str(
                c_name
                    .as_str()
                    .map_err(|_| Error::Instance(InstanceError::InvalidPhysicalDeviceName))?,
            );
            let gpu_type = prps.device_type.into();

            let queue_count = self
                .raw
                .instance
                .get_physical_device_queue_family_properties_count(gpu);

            let mut queues = Array::new();
            queues.resize_default(queue_count);

            self.raw
                .instance
                .get_physical_device_queue_family_properties(gpu, &mut queues);

            let mut queue_families = Array::new_with(a.clone());
            queue_families.reserve(queue_count);
            queue_families.extend(queues.iter().enumerate().map(|(id, q)| QueueFamily {
                queue_type: q.queue_flags.into(),
                id,
                count: q.queue_count as usize,
            }));

            let gpu_desc = hal::Gpu {
                name,
                gpu_type,
                physical_device: gpu,
                queue_families,
            };

            result_gpus.push(gpu_desc);
        }

        return Ok(result_gpus);
    }

    fn create_device(
        &self,
        gpu: VkPhysicalDevice,
        queues: &[(&QueueFamily, &[f32])],
    ) -> Result<Device, Error>
    {
        let mut queue_create_infos = Array::new();
        queue_create_infos.reserve(queues.len());

        for (family, priorities) in queues
        {
            let create_info = VkDeviceQueueCreateInfoBuilder::new()
                .queue_family_index(family.id as u32)
                .queue_count(priorities.len() as u32)
                .p_queue_priorities(priorities);

            queue_create_infos.push(create_info.build());
        }

        let create_info =
            VkDeviceCreateInfoBuilder::new().p_queue_create_infos(&queue_create_infos);

        let vk_device = self
            .raw
            .instance
            .create_device(gpu, &create_info, None)
            .map(|(_, device)| device)
            .map_err(|e| Error::Instance(InstanceError::DeviceCreationError(e)))?;

        let vk_device = vk::Device::new(vk_device, &self.raw.instance);

        Ok(Device {
            raw: Shared::new(RawDevice { device: vk_device }),
            instance: self.raw.clone(),
        })
    }
}

pub struct QueueFamily
{
    queue_type: hal::QueueType,
    id: usize,
    count: usize,
}

impl hal::QueueFamily for QueueFamily
{
    #[inline]
    fn queue_type(&self) -> hal::QueueType
    {
        self.queue_type
    }

    #[inline]
    fn id(&self) -> usize
    {
        self.id
    }

    #[inline]
    fn count(&self) -> usize
    {
        self.count
    }
}

struct RawDevice
{
    device: vk::Device,
}

impl Drop for RawDevice
{
    fn drop(&mut self)
    {
        self.device.destroy_device(None);
    }
}

pub struct Device
{
    raw: Shared<RawDevice>,
    instance: Shared<RawInstance>,
}

impl hal::Device for Device {}
