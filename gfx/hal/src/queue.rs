use crate::{capabilities::Capability, Backend};

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

pub struct Queue<B: Backend, C: Capability>
{
    pub(crate) inner: B::InnerQueue,
    pub(crate) _capability: PhantomData<C>,
}

impl<B: Backend, C: Capability> Queue<B, C>
{
    pub fn inner(&self) -> &B::InnerQueue
    {
        &self.inner
    }
}

impl<B: Backend, C: Capability> QueueFamily for Queue<B, C>
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
