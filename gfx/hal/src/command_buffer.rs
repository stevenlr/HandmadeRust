use crate::{capabilities, Backend};

use core::marker::PhantomData;

pub trait InnerCommandBuffer<B: Backend>
{
    fn reset(&mut self, release_resources: bool) -> Result<(), B::Error>;
    fn begin(&mut self) -> Result<(), B::Error>;
    fn end(&mut self) -> Result<(), B::Error>;
}

// @Todo Do we want to encode resettability in the type too?
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
    pub fn reset(&mut self, release_resources: bool) -> Result<(), B::Error>
    {
        self.inner.reset(release_resources)
    }

    pub fn end(&mut self) -> Result<(), B::Error>
    {
        self.inner.end()
    }
}

// @Todo Begin secondary command buffer

impl<B, C> CommandBuffer<B, C, capabilities::Primary>
where
    B: Backend,
    C: capabilities::Capability,
{
    pub fn begin(&mut self) -> Result<(), B::Error>
    {
        self.inner.begin()
    }
}
