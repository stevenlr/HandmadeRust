use super::hal;

use core::{ffi::c_void, mem::transmute};
use fnd::{
    alloc::Allocator,
    containers::{SmallArray8, String},
    dl::DynamicLibrary,
    str::CStr,
    Shared,
};
use vk::{self, builders::*, types::*};
use wsi;

use super::*;
use crate::conv::*;

pub(crate) struct RawInstance
{
    _dl: DynamicLibrary,
    _entry: vk::EntryPoint,
    pub(crate) instance: vk::Instance,
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
    with_surface: bool,
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

    #[inline]
    pub fn create_headless() -> Result<Self, Error>
    {
        Self::create_inner(false)
    }

    #[inline]
    pub fn create() -> Result<Self, Error>
    {
        Self::create_inner(true)
    }

    fn create_inner(with_surface: bool) -> Result<Self, Error>
    {
        let dl = DynamicLibrary::load("vulkan-1.dll")
            .ok_or(Error::Instance(InstanceError::CannotLoadLibrary))?;

        let vk_entry = vk::EntryPoint::new(|name_slice| unsafe {
            dl.get_symbol_from_bytes_null_terminated(name_slice)
                .map(|f: *mut c_void| transmute(f))
        });

        let mut layers = SmallArray8::new();
        let mut extensions = SmallArray8::new();

        if cfg!(debug_assertions)
        {
            extensions.push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr());
            layers.push(b"VK_LAYER_KHRONOS_validation\0".as_ptr());
        }

        if with_surface
        {
            extensions.push(VK_KHR_SURFACE_EXTENSION_NAME__C.as_ptr());
            extensions.push(VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C.as_ptr());
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
                _dl: dl,
                _entry: vk_entry,
                instance: vk_instance,
                debug_utils_messenger,
            }),
            with_surface,
        })
    }

    pub fn create_surface(&self, window: &wsi::Window) -> Result<Surface, Error>
    {
        Surface::create(self.raw.clone(), window)
    }
}

impl hal::Instance<Backend> for Instance
{
    fn enumerate_gpus_with<A: Allocator + Clone>(
        &self,
        a: A,
    ) -> Result<SmallArray8<hal::Gpu<Backend, A>, A>, Error>
    {
        let gpu_count = self
            .raw
            .instance
            .enumerate_physical_devices_count()
            .map_err(|e| Error::VulkanError(e))?
            .1;

        let mut gpus = SmallArray8::new();
        gpus.resize_default(gpu_count);

        self.raw
            .instance
            .enumerate_physical_devices(&mut gpus)
            .map_err(|e| Error::VulkanError(e))?;

        let mut result_gpus = SmallArray8::new_with(a.clone());
        result_gpus.reserve(gpu_count);

        for gpu in gpus.iter()
        {
            let prps = self.raw.instance.get_physical_device_properties(*gpu);
            let c_name =
                unsafe { CStr::from_bytes_null_terminated_unchecked(prps.device_name.as_ptr()) };
            let name = String::from_str(
                c_name
                    .as_str()
                    .map_err(|_| Error::Instance(InstanceError::InvalidPhysicalDeviceName))?,
            );
            let gpu_type = vk_to_hal_gpu_type(prps.device_type);

            let queue_count = self
                .raw
                .instance
                .get_physical_device_queue_family_properties_count(*gpu);

            let mut queues = SmallArray8::new();
            queues.resize_default(queue_count);

            self.raw
                .instance
                .get_physical_device_queue_family_properties(*gpu, &mut queues);

            let mut queue_families = SmallArray8::new_with(a.clone());
            queue_families.reserve(queue_count);
            queue_families.extend(queues.iter().enumerate().map(|(id, q)| QueueFamilyGroup {
                physical_device: *gpu,
                queue_type: vk_to_hal_queue_type(q.queue_flags),
                id,
                count: q.queue_count as usize,
            }));

            let gpu_desc = hal::Gpu {
                name,
                gpu_type,
                physical_device: *gpu,
                queue_families,
            };

            result_gpus.push(gpu_desc);
        }

        return Ok(result_gpus);
    }

    fn create_device<A: Allocator>(
        &self,
        gpu: &hal::Gpu<Backend, A>,
        queues: &[(&QueueFamilyGroup, &[f32])],
    ) -> Result<hal::CreatedDevice<Backend>, Error>
    {
        let mut queue_create_infos = SmallArray8::new();
        queue_create_infos.reserve(queues.len());

        for (family, priorities) in queues
        {
            let create_info = VkDeviceQueueCreateInfoBuilder::new()
                .queue_family_index(family.id as u32)
                .queue_count(priorities.len() as u32)
                .p_queue_priorities(priorities);

            queue_create_infos.push(create_info.build());
        }

        let mut extensions = SmallArray8::new();
        if self.with_surface
        {
            extensions.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME__C.as_ptr());
        }

        let create_info = VkDeviceCreateInfoBuilder::new()
            .p_queue_create_infos(&queue_create_infos)
            .pp_enabled_extension_names(&extensions);

        let vk_device = self
            .raw
            .instance
            .create_device(gpu.physical_device, &create_info, None)
            .map(|(_, device)| device)
            .map_err(|e| Error::Instance(InstanceError::DeviceCreationError(e)))?;

        let vk_device = vk::Device::new(vk_device, &self.raw.instance);

        let mut created_queues = SmallArray8::new();

        for (family, priorities) in queues
        {
            for index in 0..priorities.len()
            {
                let queue = vk_device.get_device_queue(family.id as u32, index as u32);

                created_queues.push(Some(Queue {
                    family_index: family.id,
                    queue_type: family.queue_type,
                    queue: queue,
                }));
            }
        }

        let device = Device {
            raw: Shared::new(RawDevice { device: vk_device }),
            gpu: gpu.physical_device,
            instance: self.raw.clone(),
        };

        Ok(hal::CreatedDevice {
            device: Some(device),
            queues: created_queues,
        })
    }
}
