use crate::{capabilities, Backend};

use core::marker::PhantomData;

pub trait InnerCommandBuffer
{
    fn reset(&mut self, release_resources: bool);
}

pub struct CommandBuffer<B, C, L>
where
    B: Backend,
    C: capabilities::Capability,
    L: capabilities::Level,
{
    pub(crate) inner: B::InnerCommandBuffer,
    pub(crate) _phantom: PhantomData<(C, L)>,
}

impl<B, C, L> CommandBuffer<B, C, L>
where
    B: Backend,
    C: capabilities::Capability,
    L: capabilities::Level,
{
    pub fn reset(&mut self, release_resources: bool)
    {
        self.inner.reset(release_resources);
    }
}
