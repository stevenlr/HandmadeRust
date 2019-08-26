use core::marker::PhantomData;
use fnd::{containers::SmallArray8, Shared};
use vk::{builders::*, types::*};

use crate::{conv::*, *};

#[derive(Clone, Copy, Debug)]
pub enum QueueType
{
    Graphics,
    Compute,
    Transfer,
    General,
}

#[derive(Clone, Copy)]
pub struct QueueFamily
{
    pub(crate) physical_device: VkPhysicalDevice,
    pub queue_type:             QueueType,
    pub id:                     usize,
    pub count:                  usize,
}

impl QueueFamily
{
    pub fn supports_graphics(&self) -> bool
    {
        match self.queue_type
        {
            QueueType::Graphics | QueueType::General => true,
            QueueType::Compute | QueueType::Transfer => false,
        }
    }

    pub fn supports_compute(&self) -> bool
    {
        match self.queue_type
        {
            QueueType::Compute | QueueType::General => true,
            QueueType::Graphics | QueueType::Transfer => false,
        }
    }

    pub fn supports_transfer(&self) -> bool
    {
        match self.queue_type
        {
            QueueType::Compute | QueueType::General | QueueType::Graphics | QueueType::Transfer =>
            {
                true
            }
        }
    }
}

pub struct InnerQueue
{
    pub(crate) device:       Shared<RawDevice>,
    pub(crate) family_index: usize,
    pub(crate) queue_type:   QueueType,
    pub(crate) queue:        VkQueue,
}

pub struct Queue<C>
{
    pub(crate) inner:       InnerQueue,
    pub(crate) _capability: PhantomData<C>,
}

impl<C> Queue<C>
{
    #[inline]
    pub(crate) fn raw(&self) -> VkQueue
    {
        self.inner.queue
    }

    pub fn submit<C2>(
        &mut self,
        to_wait: &[(Semaphore, PipelineStageMask)],
        cmd_buffers: &[&CommandBuffer<C2, capabilities::Primary>],
        to_signal_sem: &[Semaphore],
        to_signal_fence: Option<Fence>,
    ) -> Result<(), Error>
    where
        C2: capabilities::QueueType,
        C: capabilities::Supports<C2>,
    {
        let mut wait_sems = SmallArray8::new();
        let mut wait_stages = SmallArray8::new();
        let mut vk_cmd_buffers = SmallArray8::new();

        for c in cmd_buffers
        {
            vk_cmd_buffers.push(c.inner.raw);
        }

        for (sem, stage) in to_wait
        {
            wait_sems.push(*sem);
            wait_stages.push(hal_to_vk_pipeline_stage_flags(*stage));
        }

        let submit_info = VkSubmitInfoBuilder::new()
            .p_wait_semaphores(&wait_sems)
            .p_wait_dst_stage_mask(&wait_stages)
            .p_signal_semaphores(to_signal_sem)
            .p_command_buffers(&vk_cmd_buffers);

        let to_signal_fence = to_signal_fence.unwrap_or(VkFence::null());

        self.inner
            .device
            .device
            .queue_submit(self.inner.queue, &[submit_info.build()], to_signal_fence)
            .map(|_| ())
            .map_err(|e| Error::Vulkan(e))
    }
}
