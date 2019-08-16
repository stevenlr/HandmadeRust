use crate::*;

use core::{marker::PhantomData, ops::Range};

pub trait InnerCommandBuffer<B: Backend>
{
    fn reset(&mut self, release_resources: bool) -> Result<(), B::Error>;
    fn begin(&mut self) -> Result<(), B::Error>;
    fn end(&mut self) -> Result<(), B::Error>;

    fn cmd_pipeline_barrier<'a>(
        &mut self,
        stage: Range<PipelineStageMask>,
        barriers: &[Barrier<'a, B>],
    );

    fn cmd_clear_color_image(
        &mut self,
        image: &B::Image,
        layout: ImageLayout,
        color: ClearColor,
        range: ImageSubresourceRange,
    );
}

// @Todo Do we want to encode resettability in the type too?
pub struct CommandBuffer<B, C, L>
where
    B: Backend,
    C: capabilities::QueueType,
    L: capabilities::Level,
{
    pub(crate) inner: B::InnerCommandBuffer,
    pub(crate) _phantom: PhantomData<(C, L)>,
}

impl<B, C, L> CommandBuffer<B, C, L>
where
    B: Backend,
    C: capabilities::QueueType,
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

    pub fn cmd_pipeline_barrier<'a>(
        &mut self,
        stage: Range<PipelineStageMask>,
        barriers: &[Barrier<'a, B>],
    )
    {
        self.inner.cmd_pipeline_barrier(stage, barriers);
    }

    pub fn inner(&self) -> &B::InnerCommandBuffer
    {
        &self.inner
    }
}

// @Todo Begin secondary command buffer

impl<B, C> CommandBuffer<B, C, capabilities::Primary>
where
    B: Backend,
    C: capabilities::QueueType,
{
    pub fn begin(&mut self) -> Result<(), B::Error>
    {
        self.inner.begin()
    }
}

impl<B, C, L> CommandBuffer<B, C, L>
where
    B: Backend,
    C: capabilities::QueueType + capabilities::Supports<capabilities::GraphicsOrCompute>,
    L: capabilities::Level,
{
    pub fn cmd_clear_color_image(
        &mut self,
        image: &B::Image,
        layout: ImageLayout,
        color: ClearColor,
        range: ImageSubresourceRange,
    )
    {
        self.inner
            .cmd_clear_color_image(image, layout, color, range);
    }
}
