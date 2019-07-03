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

pub(crate) fn hal_to_vk_format(format: hal::Format) -> VkFormat
{
    match format
    {
        hal::Format::Rgb8Unorm => VkFormat::R8G8B8_UNORM,
        hal::Format::Rgb8Snorm => VkFormat::R8G8B8_SNORM,
        hal::Format::Rgb8Srgb => VkFormat::R8G8B8_SRGB,
        hal::Format::Bgr8Unorm => VkFormat::B8G8R8_UNORM,
        hal::Format::Bgr8Snorm => VkFormat::B8G8R8_SNORM,
        hal::Format::Bgr8Srgb => VkFormat::B8G8R8_SRGB,
        hal::Format::Rgba8Unorm => VkFormat::R8G8B8A8_UNORM,
        hal::Format::Rgba8Snorm => VkFormat::R8G8B8A8_SNORM,
        hal::Format::Rgba8Srgb => VkFormat::R8G8B8A8_SRGB,
        hal::Format::Bgra8Unorm => VkFormat::B8G8R8A8_UNORM,
        hal::Format::Bgra8Snorm => VkFormat::B8G8R8A8_SNORM,
        hal::Format::Bgra8Srgb => VkFormat::B8G8R8A8_SRGB,
    }
}

pub(crate) fn hal_to_vk_present_mode(present_mode: hal::PresentMode) -> VkPresentModeKHR
{
    match present_mode
    {
        hal::PresentMode::Fifo => VkPresentModeKHR::FIFO_KHR,
        hal::PresentMode::Immediate => VkPresentModeKHR::IMMEDIATE_KHR,
        hal::PresentMode::Mailbox => VkPresentModeKHR::MAILBOX_KHR,
        hal::PresentMode::Relaxed => VkPresentModeKHR::FIFO_RELAXED_KHR,
    }
}
