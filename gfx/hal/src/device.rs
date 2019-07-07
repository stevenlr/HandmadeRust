use super::{capabilities, Backend, QueueType, SwapchainConfig};

use fnd::containers::SmallArray8;

pub struct CreatedQueue<B: Backend>
{
    pub queue_type: QueueType,
    pub queue: Option<B::Queue>,
}

pub struct CreatedDevice<B: Backend>
{
    pub device: Option<B::Device>,
    pub queues: SmallArray8<CreatedQueue<B>>,
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
        core::mem::replace(&mut self.device, None).ok_or(QueueRetrievalError::AlreadyRetrieved)
    }

    pub fn retrieve_queue<C>(&mut self, index: usize) -> Result<B::Queue, QueueRetrievalError>
    where
        C: capabilities::Capability,
    {
        if index >= self.queues.len()
        {
            return Err(QueueRetrievalError::QueueIndexOutOfBounds);
        }

        if self.queues[index].queue.is_some()
        {
            if C::supported_by(self.queues[index].queue_type)
            {
                core::mem::replace(&mut self.queues[index].queue, None)
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

pub trait Device<B: Backend>
{
    fn create_swapchain(
        &self,
        surface: &B::Surface,
        config: &SwapchainConfig<B>,
    ) -> Result<B::Swapchain, B::Error>;
    fn destroy_swapchain(&self, swapchain: B::Swapchain);
}
