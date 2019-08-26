use crate::*;
use vk::types::*;

pub(crate) fn vk_to_hal_gpu_type(vk_type: VkPhysicalDeviceType) -> GpuType
{
    match vk_type
    {
        VkPhysicalDeviceType::CPU => GpuType::Cpu,
        VkPhysicalDeviceType::DISCRETE_GPU => GpuType::DiscreteGpu,
        VkPhysicalDeviceType::INTEGRATED_GPU => GpuType::IntegratedGpu,
        VkPhysicalDeviceType::VIRTUAL_GPU => GpuType::VirtualGpu,
        _ => GpuType::Unknown,
    }
}

pub(crate) fn vk_to_hal_queue_type(vk_flags: VkQueueFlags) -> QueueType
{
    let has_graphics = vk_flags.contains(VkQueueFlags::GRAPHICS_BIT);
    let has_compute = vk_flags.contains(VkQueueFlags::COMPUTE_BIT);
    let has_transfer = vk_flags.contains(VkQueueFlags::TRANSFER_BIT);

    match (has_graphics, has_compute, has_transfer)
    {
        (true, true, _) => QueueType::General,
        (true, false, _) => QueueType::Graphics,
        (false, true, _) => QueueType::Compute,
        (false, false, true) => QueueType::Transfer,
        _ => unreachable!(),
    }
}

pub(crate) fn hal_to_vk_format(format: Format) -> VkFormat
{
    use Format::*;

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

pub(crate) fn hal_to_vk_present_mode(present_mode: PresentMode) -> VkPresentModeKHR
{
    use PresentMode::*;

    match present_mode
    {
        Fifo => VkPresentModeKHR::FIFO_KHR,
        Immediate => VkPresentModeKHR::IMMEDIATE_KHR,
        Mailbox => VkPresentModeKHR::MAILBOX_KHR,
        Relaxed => VkPresentModeKHR::FIFO_RELAXED_KHR,
    }
}

pub(crate) fn hal_to_vk_image_layout(layout: ImageLayout) -> VkImageLayout
{
    use ImageLayout::*;

    match layout
    {
        Undefined => VkImageLayout::UNDEFINED,
        General => VkImageLayout::GENERAL,
        ColorAttachmentOptimal => VkImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        DepthStencilAttachmentOptimal => VkImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        DepthStencilReadOnlyOptimal => VkImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
        ShaderReadOnlyOptimal => VkImageLayout::SHADER_READ_ONLY_OPTIMAL,
        TransferSrcOptimal => VkImageLayout::TRANSFER_SRC_OPTIMAL,
        TransferDstOptimal => VkImageLayout::TRANSFER_DST_OPTIMAL,
        Preinitialized => VkImageLayout::PREINITIALIZED,
        PresentSrc => VkImageLayout::PRESENT_SRC_KHR,
    }
}

pub(crate) fn hal_to_vk_clear_color(color: &ClearColor) -> VkClearColorValue
{
    match color
    {
        ClearColor::Float(c) => VkClearColorValue { float_32: *c },
        ClearColor::Int32(c) => VkClearColorValue { int_32: *c },
        ClearColor::UInt32(c) => VkClearColorValue { uint_32: *c },
    }
}

pub(crate) fn hal_to_vk_image_subresource_range(
    range: &ImageSubresourceRange,
) -> VkImageSubresourceRange
{
    VkImageSubresourceRange {
        aspect_mask:      hal_to_vk_image_aspect_flags(range.aspects),
        base_mip_level:   range.mip_levels.start,
        level_count:      range.mip_levels.end - range.mip_levels.start + 1,
        base_array_layer: range.layers.start,
        layer_count:      range.layers.end - range.layers.start + 1,
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
hal_to_vk_flags!(hal_to_vk_command_pool_flags, CommandPoolFlags, VkCommandPoolCreateFlags,
    TRANSIENT => TRANSIENT_BIT,
    RESET_COMMAND_BUFFER => RESET_COMMAND_BUFFER_BIT,
);

hal_to_vk_flags!(hal_to_vk_pipeline_stage_flags, PipelineStageMask, VkPipelineStageFlags,
    TOP_OF_PIPE => TOP_OF_PIPE_BIT,
    DRAW_INDIRECT => DRAW_INDIRECT_BIT,
    VERTEX_INPUT => VERTEX_INPUT_BIT,
    VERTEX_SHADER => VERTEX_SHADER_BIT,
    TESSELLATION_CONTROL_SHADER => TESSELLATION_CONTROL_SHADER_BIT,
    TESSELLATION_EVALUATION_SHADER => TESSELLATION_EVALUATION_SHADER_BIT,
    GEOMETRY_SHADER => GEOMETRY_SHADER_BIT,
    FRAGMENT_SHADER => FRAGMENT_SHADER_BIT,
    EARLY_FRAGMENT_TESTS => EARLY_FRAGMENT_TESTS_BIT,
    LATE_FRAGMENT_TESTS => LATE_FRAGMENT_TESTS_BIT,
    COLOR_ATTACHMENT_OUTPUT => COLOR_ATTACHMENT_OUTPUT_BIT,
    COMPUTE_SHADER => COMPUTE_SHADER_BIT,
    TRANSFER => TRANSFER_BIT,
    BOTTOM_OF_PIPE => BOTTOM_OF_PIPE_BIT,
    HOST => HOST_BIT,
    ALL_GRAPHICS => ALL_GRAPHICS_BIT,
    ALL_COMMANDS => ALL_COMMANDS_BIT,
);

hal_to_vk_flags!(hal_to_vk_access_mask_flags, AccessMask, VkAccessFlags,
    INDIRECT_COMMAND_READ => INDIRECT_COMMAND_READ_BIT,
    INDEX_READ => INDEX_READ_BIT,
    VERTEX_ATTRIBUTE_READ => VERTEX_ATTRIBUTE_READ_BIT,
    UNIFORM_READ => UNIFORM_READ_BIT,
    INPUT_ATTACHMENT_READ => INPUT_ATTACHMENT_READ_BIT,
    SHADER_READ => SHADER_READ_BIT,
    SHADER_WRITE => SHADER_WRITE_BIT,
    COLOR_ATTACHMENT_READ => COLOR_ATTACHMENT_READ_BIT,
    COLOR_ATTACHMENT_WRITE => COLOR_ATTACHMENT_WRITE_BIT,
    DEPTH_STENCIL_ATTACHMENT_READ => DEPTH_STENCIL_ATTACHMENT_READ_BIT,
    DEPTH_STENCIL_ATTACHMENT_WRITE => DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
    TRANSFER_READ => TRANSFER_READ_BIT,
    TRANSFER_WRITE => TRANSFER_WRITE_BIT,
    HOST_READ => HOST_READ_BIT,
    HOST_WRITE => HOST_WRITE_BIT,
    MEMORY_READ => MEMORY_READ_BIT,
    MEMORY_WRITE => MEMORY_WRITE_BIT,
);

hal_to_vk_flags!(hal_to_vk_image_aspect_flags, ImageAspectMask, VkImageAspectFlags,
    COLOR => COLOR_BIT,
    DEPTH => DEPTH_BIT,
    STENCIL => STENCIL_BIT,
);
