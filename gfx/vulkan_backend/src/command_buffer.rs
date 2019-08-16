use core::ops::Range;
use fnd::{containers::SmallArray8, Shared};
use gfx_hal as hal;
use vk::{builders::*, types::*};

use super::{conv::*, Backend, Error, RawDevice};

pub struct CommandBuffer
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) raw:    VkCommandBuffer,
}

impl hal::InnerCommandBuffer<Backend> for CommandBuffer
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
            .map_err(|e| Error::VulkanError(e))
    }

    fn begin(&mut self) -> Result<(), Error>
    {
        let begin_info = VkCommandBufferBeginInfoBuilder::new();
        self.device
            .device
            .begin_command_buffer(self.raw, &begin_info)
            .map(|_| ())
            .map_err(|e| Error::VulkanError(e))
    }

    fn end(&mut self) -> Result<(), Error>
    {
        self.device
            .device
            .end_command_buffer(self.raw)
            .map(|_| ())
            .map_err(|e| Error::VulkanError(e))
    }

    fn cmd_pipeline_barrier<'a>(
        &mut self,
        stage: Range<hal::PipelineStageMask>,
        barriers: &[hal::Barrier<'a, Backend>],
    )
    {
        let src_stage_mask = hal_to_vk_pipeline_stage_flags(stage.start);
        let dst_stage_mask = hal_to_vk_pipeline_stage_flags(stage.end);

        let mut global_barriers = SmallArray8::new();
        let mut image_barriers = SmallArray8::new();

        for barrier in barriers
        {
            match barrier
            {
                hal::Barrier::Global(range) =>
                {
                    global_barriers.push(
                        VkMemoryBarrierBuilder::new()
                            .src_access_mask(hal_to_vk_access_mask_flags(range.start))
                            .dst_access_mask(hal_to_vk_access_mask_flags(range.end))
                            .build(),
                    );
                }
                hal::Barrier::Image {
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
                            .image(**image)
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
        image: &VkImage,
        layout: hal::ImageLayout,
        color: hal::ClearColor,
        range: hal::ImageSubresourceRange,
    )
    {
        let range = hal_to_vk_image_subresource_range(&range);
        let layout = hal_to_vk_image_layout(layout);
        let color = hal_to_vk_clear_color(&color);

        self.device
            .device
            .cmd_clear_color_image(self.raw, *image, layout, &color, &[range]);
    }
}
