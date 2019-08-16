use fnd::Shared;
use gfx_hal as hal;
use vk::types::*;

use super::RawDevice;

pub struct CommandBuffer
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) raw: VkCommandBuffer,
}

impl hal::InnerCommandBuffer for CommandBuffer
{
    fn reset(&mut self, release_resources: bool)
    {
        let flags = if release_resources
        {
            VkCommandBufferResetFlagBits::RELEASE_RESOURCES_BIT
        }
        else
        {
            VkCommandBufferResetFlagBits::default()
        };

        drop(self.device.device.reset_command_buffer(self.raw, flags));
    }
}
