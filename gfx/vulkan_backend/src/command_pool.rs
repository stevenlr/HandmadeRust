use super::RawDevice;

use fnd::Shared;
use gfx_hal as hal;
use vk::types::*;

pub struct CommandPool
{
    pub(crate) raw: VkCommandPool,
    pub(crate) device: Shared<RawDevice>,
}

impl hal::InnerCommandPool for CommandPool
{
    fn reset(&self, release_resources: bool)
    {
        let flags = if release_resources
        {
            VkCommandPoolResetFlagBits::RELEASE_RESOURCES_BIT
        }
        else
        {
            VkCommandPoolResetFlagBits::default()
        };

        core::mem::forget(self.device.device.reset_command_pool(self.raw, flags));
    }
}
