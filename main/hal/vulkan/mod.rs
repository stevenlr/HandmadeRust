use vk::{self, builders::*, types::*};

use fnd::{
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

pub struct Backend;

impl hal::Backend for Backend
{
    type Instance = Instance;
    type PhysicalDevice = VkPhysicalDevice;
}

pub struct RawInstance
{
    dl: DynamicLibrary,
    entry: vk::EntryPoint,
    instance: vk::Instance,
    gpus: Array<hal::Gpu<Backend>>,
}

impl Drop for RawInstance
{
    fn drop(&mut self)
    {
        self.instance.destroy_instance(None);
    }
}

#[derive(Debug)]
pub enum InstanceError
{
    CannotLoadLibrary,
    CreationError(VkResult),
    VulkanError(VkResult),
    InvalidPhysicalDeviceName,
}

pub struct Instance
{
    raw: Shared<RawInstance>,
}

impl Instance
{
    pub fn create() -> Result<Self, InstanceError>
    {
        let dl = DynamicLibrary::load("vulkan-1.dll").ok_or(InstanceError::CannotLoadLibrary)?;
        let vk_entry = vk::EntryPoint::new(|name_slice| unsafe {
            dl.get_symbol_from_bytes_null_terminated(name_slice)
                .map(|f: *mut c_void| transmute(f))
        });

        let app_info = VkApplicationInfoBuilder::new();
        let create_info = VkInstanceCreateInfoBuilder::new().p_application_info(Some(&app_info));

        let vk_instance = vk_entry
            .create_instance(&create_info, None)
            .map(|(_, instance)| instance)
            .map_err(|vk_result| InstanceError::CreationError(vk_result))?;

        let vk_instance = vk::Instance::new(vk_instance, &vk_entry);

        let gpu_count = vk_instance
            .enumerate_physical_devices_count()
            .map_err(|e| InstanceError::VulkanError(e))?
            .1;

        let mut gpus = Array::new();
        gpus.resize_default(gpu_count);

        vk_instance
            .enumerate_physical_devices(&mut gpus)
            .map_err(|e| InstanceError::VulkanError(e))?;

        let mut result_gpus = Array::new();
        result_gpus.reserve(gpu_count);

        for gpu in gpus
        {
            let prps = vk_instance.get_physical_device_properties(gpu);
            let c_name =
                unsafe { CStr::from_bytes_null_terminated_unchecked(prps.device_name.as_ptr()) };
            let name = String::from_str(
                c_name
                    .as_str()
                    .map_err(|_| InstanceError::InvalidPhysicalDeviceName)?,
            );
            let gpu_type = prps.device_type.into();

            let gpu_desc = hal::Gpu {
                name,
                gpu_type,
                physical_device: gpu,
            };

            result_gpus.push(gpu_desc);
        }

        Ok(Self {
            raw: Shared::new(RawInstance {
                dl,
                entry: vk_entry,
                instance: vk_instance,
                gpus: result_gpus,
            }),
        })
    }
}

impl hal::Instance<Backend> for Instance
{
    fn enumerate_gpus(&self) -> &[hal::Gpu<Backend>]
    {
        &self.raw.gpus
    }
}
