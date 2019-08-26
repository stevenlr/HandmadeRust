use core::{marker::PhantomData, ops::Range};
use fnd::{containers::SmallArray8, Shared};
use vk::{builders::*, types::*};

use super::{capabilities, conv::*, *};

pub(crate) struct InnerCommandBuffer
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) raw:    VkCommandBuffer,
}

impl InnerCommandBuffer
{
    fn reset(&mut self, release_resources: bool) -> Result<(), Error>
    {
        let flags = if release_resources
        {
            VkCommandBufferResetFlagBits::RELEASE_RESOURCES_BIT
        }
        else
        {
            VkCommandBufferResetFlagBits::default()
        };

        self.device
            .device
            .reset_command_buffer(self.raw, flags)
            .map(|_| ())
            .map_err(|e| Error::Vulkan(e))
    }

    fn begin(&mut self) -> Result<(), Error>
    {
        let begin_info = VkCommandBufferBeginInfoBuilder::new();
        self.device
            .device
            .begin_command_buffer(self.raw, &begin_info)
            .map(|_| ())
            .map_err(|e| Error::Vulkan(e))
    }

    fn end(&mut self) -> Result<(), Error>
    {
        self.device
            .device
            .end_command_buffer(self.raw)
            .map(|_| ())
            .map_err(|e| Error::Vulkan(e))
    }

    fn cmd_pipeline_barrier(&mut self, stage: Range<PipelineStageMask>, barriers: &[Barrier])
    {
        let src_stage_mask = hal_to_vk_pipeline_stage_flags(stage.start);
        let dst_stage_mask = hal_to_vk_pipeline_stage_flags(stage.end);

        let mut global_barriers = SmallArray8::new();
        let mut image_barriers = SmallArray8::new();

        for barrier in barriers
        {
            match barrier
            {
                Barrier::Global(range) =>
                {
                    global_barriers.push(
                        VkMemoryBarrierBuilder::new()
                            .src_access_mask(hal_to_vk_access_mask_flags(range.start))
                            .dst_access_mask(hal_to_vk_access_mask_flags(range.end))
                            .build(),
                    );
                }
                Barrier::Image {
                    access,
                    layout,
                    queues,
                    image,
                    range,
                } =>
                {
                    let queues = queues.clone().unwrap_or(0..0);

                    image_barriers.push(
                        VkImageMemoryBarrierBuilder::new()
                            .src_access_mask(hal_to_vk_access_mask_flags(access.start))
                            .dst_access_mask(hal_to_vk_access_mask_flags(access.end))
                            .old_layout(hal_to_vk_image_layout(layout.start))
                            .new_layout(hal_to_vk_image_layout(layout.end))
                            .src_queue_family_index(queues.start)
                            .dst_queue_family_index(queues.end)
                            .image(*image)
                            .subresource_range(hal_to_vk_image_subresource_range(range))
                            .build(),
                    );
                }
            }
        }

        self.device.device.cmd_pipeline_barrier(
            self.raw,
            src_stage_mask,
            dst_stage_mask,
            VkDependencyFlags::default(),
            &global_barriers,
            &[],
            &image_barriers,
        );
    }

    fn cmd_clear_color_image(
        &mut self,
        image: Image,
        layout: ImageLayout,
        color: ClearColor,
        range: ImageSubresourceRange,
    )
    {
        let range = hal_to_vk_image_subresource_range(&range);
        let layout = hal_to_vk_image_layout(layout);
        let color = hal_to_vk_clear_color(&color);

        self.device
            .device
            .cmd_clear_color_image(self.raw, image, layout, &color, &[range]);
    }
}

// @Todo Do we want to encode resettability in the type too?
pub struct CommandBuffer<C, L>
where
    C: capabilities::QueueType,
    L: capabilities::Level,
{
    pub(crate) inner:    InnerCommandBuffer,
    pub(crate) _phantom: PhantomData<(C, L)>,
}

impl<C, L> CommandBuffer<C, L>
where
    C: capabilities::QueueType,
    L: capabilities::Level,
{
    pub fn reset(&mut self, release_resources: bool) -> Result<(), Error>
    {
        self.inner.reset(release_resources)
    }

    pub fn end(&mut self) -> Result<(), Error>
    {
        self.inner.end()
    }

    pub fn cmd_pipeline_barrier(&mut self, stage: Range<PipelineStageMask>, barriers: &[Barrier])
    {
        self.inner.cmd_pipeline_barrier(stage, barriers);
    }
}

// @Todo Begin secondary command buffer

impl<C> CommandBuffer<C, capabilities::Primary>
where
    C: capabilities::QueueType,
{
    pub fn begin(&mut self) -> Result<(), Error>
    {
        self.inner.begin()
    }
}

impl<C, L> CommandBuffer<C, L>
where
    C: capabilities::QueueType + capabilities::Supports<capabilities::GraphicsOrCompute>,
    L: capabilities::Level,
{
    pub fn cmd_clear_color_image(
        &mut self,
        image: Image,
        layout: ImageLayout,
        color: ClearColor,
        range: ImageSubresourceRange,
    )
    {
        self.inner
            .cmd_clear_color_image(image, layout, color, range);
    }
}
