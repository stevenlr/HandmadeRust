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
    use hal::Format::*;

    match format
    {
        Rgb8Unorm => VkFormat::R8G8B8_UNORM,
        Rgb8Snorm => VkFormat::R8G8B8_SNORM,
        Rgb8Srgb => VkFormat::R8G8B8_SRGB,
        Bgr8Unorm => VkFormat::B8G8R8_UNORM,
        Bgr8Snorm => VkFormat::B8G8R8_SNORM,
        Bgr8Srgb => VkFormat::B8G8R8_SRGB,
        Rgba8Unorm => VkFormat::R8G8B8A8_UNORM,
        Rgba8Snorm => VkFormat::R8G8B8A8_SNORM,
        Rgba8Srgb => VkFormat::R8G8B8A8_SRGB,
        Bgra8Unorm => VkFormat::B8G8R8A8_UNORM,
        Bgra8Snorm => VkFormat::B8G8R8A8_SNORM,
        Bgra8Srgb => VkFormat::B8G8R8A8_SRGB,
    }
}

pub(crate) fn hal_to_vk_present_mode(present_mode: hal::PresentMode) -> VkPresentModeKHR
{
    use hal::PresentMode::*;

    match present_mode
    {
        Fifo => VkPresentModeKHR::FIFO_KHR,
        Immediate => VkPresentModeKHR::IMMEDIATE_KHR,
        Mailbox => VkPresentModeKHR::MAILBOX_KHR,
        Relaxed => VkPresentModeKHR::FIFO_RELAXED_KHR,
    }
}

pub(crate) fn hal_to_vk_command_buffer_level(
    level: hal::InnerCommandBufferLevel,
) -> VkCommandBufferLevel
{
    use hal::InnerCommandBufferLevel::*;

    match level
    {
        Primary => VkCommandBufferLevel::PRIMARY,
        Secondary => VkCommandBufferLevel::SECONDARY,
    }
}

macro_rules! hal_to_vk_flags {
    ($name:ident, $from_type:ty, $to_type:ty, $($from:ident => $to:ident,)+) =>
    {
        pub(crate) fn $name(src: $from_type) -> $to_type
        {
            let mut result = <$to_type>::default();

            $(
                if src.contains(<$from_type>::$from)
                {
                    result |= <$to_type>::$to;
                }
            )+

            return result;
        }
    };
}

#[rustfmt::skip]
hal_to_vk_flags!(hal_to_vk_command_pool_flags, hal::CommandPoolFlags, VkCommandPoolCreateFlags, 
    TRANSIENT => TRANSIENT_BIT,
    RESET_COMMAND_BUFFER => RESET_COMMAND_BUFFER_BIT,
);
