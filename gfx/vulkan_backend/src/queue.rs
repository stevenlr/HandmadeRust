use fnd::{containers::SmallArray8, Shared};
use gfx_hal as hal;
use vk::{builders::*, types::*};

use crate::{conv::*, Backend, Error, RawDevice};

#[derive(Clone, Copy)]
pub struct QueueFamilyGroup
{
    pub(crate) physical_device: VkPhysicalDevice,
    pub(crate) queue_type: hal::QueueType,
    pub(crate) id: usize,
    pub(crate) count: usize,
}

impl hal::QueueFamily for QueueFamilyGroup
{
    #[inline]
    fn queue_type(&self) -> hal::QueueType
    {
        self.queue_type
    }

    #[inline]
    fn id(&self) -> usize
    {
        self.id
    }
}

impl hal::QueueFamilyGroup for QueueFamilyGroup
{
    #[inline]
    fn count(&self) -> usize
    {
        self.count
    }
}

pub struct Queue
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) family_index: usize,
    pub(crate) queue_type: hal::QueueType,
    pub(crate) queue: VkQueue,
}

impl hal::QueueFamily for Queue
{
    #[inline]
    fn queue_type(&self) -> hal::QueueType
    {
        self.queue_type
    }

    #[inline]
    fn id(&self) -> usize
    {
        self.family_index
    }
}

impl hal::InnerQueue<Backend> for Queue
{
    fn submit<C>(
        &mut self,
        to_wait: &[(VkSemaphore, hal::PipelineStageMask)],
        cmd_buffers: &[&hal::CommandBuffer<Backend, C, hal::capabilities::Primary>],
        to_signal_sem: &[VkSemaphore],
        to_signal_fence: Option<VkFence>,
    ) -> Result<(), Error>
    where
        C: hal::capabilities::QueueType,
    {
        let mut wait_sems = SmallArray8::new();
        let mut wait_stages = SmallArray8::new();
        let mut vk_cmd_buffers = SmallArray8::new();

        for c in cmd_buffers
        {
            vk_cmd_buffers.push(c.inner().raw);
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

        self.device
            .device
            .queue_submit(self.queue, &[submit_info.build()], to_signal_fence)
            .map(|_| ())
            .map_err(|e| Error::VulkanError(e))
    }
}
