use crate::{capabilities, *};

use core::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub enum QueueType
{
    Graphics,
    Compute,
    Transfer,
    General,
}

pub trait QueueFamily
{
    fn queue_type(&self) -> QueueType;
    fn id(&self) -> usize;

    fn supports_graphics(&self) -> bool
    {
        match self.queue_type()
        {
            QueueType::Graphics | QueueType::General => true,
            QueueType::Compute | QueueType::Transfer => false,
        }
    }

    fn supports_compute(&self) -> bool
    {
        match self.queue_type()
        {
            QueueType::Compute | QueueType::General => true,
            QueueType::Graphics | QueueType::Transfer => false,
        }
    }

    fn supports_transfer(&self) -> bool
    {
        match self.queue_type()
        {
            QueueType::Compute | QueueType::General | QueueType::Graphics | QueueType::Transfer =>
            {
                true
            }
        }
    }
}

pub trait QueueFamilyGroup: QueueFamily
{
    fn count(&self) -> usize;
}

pub struct Queue<B: Backend, C>
{
    pub(crate) inner: B::InnerQueue,
    pub(crate) _capability: PhantomData<C>,
}

impl<B: Backend, C: capabilities::QueueType> Queue<B, C>
{
    pub fn inner(&self) -> &B::InnerQueue
    {
        &self.inner
    }

    pub fn submit<C2>(
        &mut self,
        to_wait: &[(B::Semaphore, PipelineStageMask)],
        cmd_buffers: &[&CommandBuffer<B, C2, capabilities::Primary>],
        to_signal_sem: &[B::Semaphore],
        to_signal_fence: Option<B::Fence>,
    ) -> Result<(), B::Error>
    where
        C2: capabilities::QueueType,
        C: capabilities::Supports<C2>,
    {
        self.inner
            .submit(to_wait, cmd_buffers, to_signal_sem, to_signal_fence)
    }
}

impl<B: Backend, C: capabilities::QueueType> QueueFamily for Queue<B, C>
{
    #[inline]
    fn queue_type(&self) -> QueueType
    {
        self.inner.queue_type()
    }

    #[inline]
    fn id(&self) -> usize
    {
        self.inner.id()
    }
}

pub trait InnerQueue<B: Backend>: QueueFamily
{
    fn submit<C>(
        &mut self,
        to_wait: &[(B::Semaphore, PipelineStageMask)],
        cmd_buffers: &[&CommandBuffer<B, C, capabilities::Primary>],
        to_signal_sem: &[B::Semaphore],
        to_signal_fence: Option<B::Fence>,
    ) -> Result<(), B::Error>
    where
        C: capabilities::QueueType;
}
