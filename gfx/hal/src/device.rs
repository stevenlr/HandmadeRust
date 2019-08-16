use super::{capabilities, Backend, CommandPool, Queue, QueueFamily, SwapchainConfig};

use core::marker::PhantomData;
use fnd::{bitflags, containers::SmallArray8};

pub struct CreatedDevice<B: Backend>
{
    pub device: Option<B::Device>,
    pub queues: SmallArray8<Option<B::InnerQueue>>,
}

#[derive(Debug)]
pub enum QueueRetrievalError
{
    QueueIndexOutOfBounds,
    AlreadyRetrieved,
    IncompatibleCapabilities,
}

impl<B: Backend> CreatedDevice<B>
{
    pub fn retrieve_device(&mut self) -> Result<B::Device, QueueRetrievalError>
    {
        self.device
            .take()
            .ok_or(QueueRetrievalError::AlreadyRetrieved)
    }

    pub fn retrieve_queue<C>(&mut self, index: usize) -> Result<Queue<B, C>, QueueRetrievalError>
    where
        C: capabilities::Capability,
    {
        if index >= self.queues.len()
        {
            return Err(QueueRetrievalError::QueueIndexOutOfBounds);
        }

        let queue_slot = &mut self.queues[index];
        if queue_slot.is_some()
        {
            if C::supported_by(queue_slot.as_ref().unwrap().queue_type())
            {
                queue_slot
                    .take()
                    .map(|q| Queue {
                        inner: q,
                        _capability: PhantomData,
                    })
                    .ok_or(QueueRetrievalError::AlreadyRetrieved)
            }
            else
            {
                Err(QueueRetrievalError::IncompatibleCapabilities)
            }
        }
        else
        {
            Err(QueueRetrievalError::AlreadyRetrieved)
        }
    }
}

bitflags! {
    pub enum CommandPoolFlags: u8 {
        TRANSIENT = 1,
        RESET_COMMAND_BUFFER = 2,
    }
}

pub trait Device<B: Backend>
{
    fn create_swapchain(
        &self,
        surface: &B::Surface,
        config: &SwapchainConfig<B>,
    ) -> Result<B::Swapchain, B::Error>;

    fn destroy_swapchain(&self, swapchain: B::Swapchain);

    fn create_command_pool<C: capabilities::Capability>(
        &self,
        queue: &Queue<B, C>,
        flags: CommandPoolFlags,
    ) -> Result<CommandPool<B, C>, B::Error>;

    fn destroy_command_pool<C: capabilities::Capability>(&self, pool: CommandPool<B, C>);

    fn wait_idle(&self);
}
