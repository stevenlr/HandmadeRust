use gfx_hal as hal;
use vk::types::*;

pub(crate) fn vk_to_hal_gpu_type(vk_type: VkPhysicalDeviceType) -> hal::GpuType
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

pub(crate) fn vk_to_hal_queue_type(vk_flags: VkQueueFlags) -> hal::QueueType
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
