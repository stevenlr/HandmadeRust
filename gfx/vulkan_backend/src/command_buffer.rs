use fnd::Shared;
use gfx_hal as hal;
use vk::{builders::*, types::*};

use super::{Backend, Error, RawDevice};

pub struct CommandBuffer
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) raw: VkCommandBuffer,
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
}
