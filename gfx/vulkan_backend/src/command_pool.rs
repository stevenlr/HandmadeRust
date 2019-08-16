use super::{Backend, CommandBuffer, Error, RawDevice};

use fnd::Shared;
use gfx_hal as hal;
use vk::{builders::*, types::*};

use crate::conv::*;

pub struct CommandPool
{
    pub(crate) raw: VkCommandPool,
    pub(crate) device: Shared<RawDevice>,
}

impl hal::InnerCommandPool<Backend> for CommandPool
{
    fn reset(&mut self, release_resources: bool)
    {
        let flags = if release_resources
        {
            VkCommandPoolResetFlagBits::RELEASE_RESOURCES_BIT
        }
        else
        {
            VkCommandPoolResetFlagBits::default()
        };

        drop(self.device.device.reset_command_pool(self.raw, flags));
    }

    fn alloc_command_buffer(
        &mut self,
        level: hal::InnerCommandBufferLevel,
    ) -> Result<CommandBuffer, Error>
    {
        let alloc_info = VkCommandBufferAllocateInfoBuilder::new()
            .command_pool(self.raw)
            .level(hal_to_vk_command_buffer_level(level))
            .command_buffer_count(1);

        let mut cmd_buffer = [VkCommandBuffer::null(); 1];

        self.device
            .device
            .allocate_command_buffers(&alloc_info, &mut cmd_buffer[0..1])
            .map_err(|e| Error::CommandPool(e))?;

        return Ok(CommandBuffer {
            device: self.device.clone(),
            raw: cmd_buffer[0],
        });
    }

    fn free_command_buffer(&mut self, cmd_buffer: CommandBuffer)
    {
        self.device
            .device
            .free_command_buffers(self.raw, &[cmd_buffer.raw]);
    }
}
