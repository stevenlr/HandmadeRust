use super::Backend;

use core::marker::PhantomData;

pub trait InnerCommandPool
{
    fn reset(&self, release_resources: bool);
}

pub struct CommandPool<B: Backend, C>
{
    inner: B::InnerCommandPool,
    _capability: PhantomData<C>,
}

impl<B: Backend, C> CommandPool<B, C>
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

    pub fn reset(&self, release_resources: bool)
    {
        self.inner.reset(release_resources);
    }
}
