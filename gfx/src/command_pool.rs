use super::{capabilities, CommandBuffer, Error, InnerCommandBuffer, RawDevice};

use core::marker::PhantomData;
use fnd::Shared;
use vk::{builders::*, types::*};

pub(crate) struct InnerCommandPool
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) raw:    VkCommandPool,
}

impl InnerCommandPool
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
        level: VkCommandBufferLevel,
    ) -> Result<InnerCommandBuffer, Error>
    {
        let alloc_info = VkCommandBufferAllocateInfoBuilder::new()
            .command_pool(self.raw)
            .level(level)
            .command_buffer_count(1);

        let mut cmd_buffer = [VkCommandBuffer::null(); 1];

        self.device
            .device
            .allocate_command_buffers(&alloc_info, &mut cmd_buffer[0..1])
            .map_err(|e| Error::Vulkan(e))?;

        return Ok(InnerCommandBuffer {
            device: self.device.clone(),
            raw:    cmd_buffer[0],
        });
    }

    fn free_command_buffer(&mut self, cmd_buffer: InnerCommandBuffer)
    {
        self.device
            .device
            .free_command_buffers(self.raw, &[cmd_buffer.raw]);
    }
}

pub struct CommandPool<C>
{
    pub(crate) inner: InnerCommandPool,
    _capability:      PhantomData<C>,
}

impl<C> CommandPool<C>
where
    C: capabilities::QueueType,
{
    pub(crate) fn new(inner: InnerCommandPool) -> Self
    {
        CommandPool {
            inner,
            _capability: PhantomData,
        }
    }

    pub fn reset(&mut self, release_resources: bool)
    {
        self.inner.reset(release_resources);
    }

    pub fn alloc_primary_command_buffer(
        &mut self,
    ) -> Result<CommandBuffer<C, capabilities::Primary>, Error>
    {
        self.inner
            .alloc_command_buffer(VkCommandBufferLevel::PRIMARY)
            .map(|b| CommandBuffer {
                inner:    b,
                _phantom: PhantomData,
            })
    }

    pub fn alloc_secondary_command_buffer(
        &mut self,
    ) -> Result<CommandBuffer<C, capabilities::Secondary>, Error>
    {
        self.inner
            .alloc_command_buffer(VkCommandBufferLevel::SECONDARY)
            .map(|b| CommandBuffer {
                inner:    b,
                _phantom: PhantomData,
            })
    }

    pub fn free_command_buffer<L>(&mut self, cmd_buffer: CommandBuffer<C, L>)
    where
        L: capabilities::Level,
    {
        self.inner.free_command_buffer(cmd_buffer.inner);
    }
}
