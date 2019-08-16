use super::{capabilities, Backend, CommandBuffer};

use core::marker::PhantomData;

pub enum InnerCommandBufferLevel
{
    Primary,
    Secondary,
}

pub trait InnerCommandPool<B: Backend>
{
    fn alloc_command_buffer(
        &mut self,
        level: InnerCommandBufferLevel,
    ) -> Result<B::InnerCommandBuffer, B::Error>;
    fn free_command_buffer(&mut self, cmd_buffer: B::InnerCommandBuffer);
    fn reset(&mut self, release_resources: bool);
}

pub struct CommandPool<B: Backend, C>
{
    inner: B::InnerCommandPool,
    _capability: PhantomData<C>,
}

impl<B: Backend, C> CommandPool<B, C>
where
    C: capabilities::QueueType,
{
    pub fn new(inner: B::InnerCommandPool) -> Self
    {
        CommandPool {
            inner,
            _capability: PhantomData,
        }
    }

    pub fn into_inner(self) -> B::InnerCommandPool
    {
        self.inner
    }

    pub fn reset(&mut self, release_resources: bool)
    {
        self.inner.reset(release_resources);
    }

    pub fn alloc_primary_command_buffer(
        &mut self,
    ) -> Result<CommandBuffer<B, C, capabilities::Primary>, B::Error>
    {
        self.inner
            .alloc_command_buffer(InnerCommandBufferLevel::Primary)
            .map(|b| CommandBuffer {
                inner: b,
                _phantom: PhantomData,
            })
    }

    pub fn alloc_secondary_command_buffer(
        &mut self,
    ) -> Result<CommandBuffer<B, C, capabilities::Secondary>, B::Error>
    {
        self.inner
            .alloc_command_buffer(InnerCommandBufferLevel::Secondary)
            .map(|b| CommandBuffer {
                inner: b,
                _phantom: PhantomData,
            })
    }

    pub fn free_command_buffer<L>(&mut self, cmd_buffer: CommandBuffer<B, C, L>)
    where
        L: capabilities::Level,
    {
        self.inner.free_command_buffer(cmd_buffer.inner);
    }
}
