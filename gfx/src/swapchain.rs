use crate::{capabilities, Error, Fence, Format, Image, Queue, QueueFamily, RawDevice, Semaphore};

use fnd::{containers::SmallArray8, Shared};
use vk::{builders::*, types::*};

#[derive(Copy, Clone)]
pub enum PresentMode
{
    Immediate,
    Mailbox,
    Fifo,
    Relaxed,
}

pub struct SwapchainConfig<'a>
{
    pub queue_family: &'a QueueFamily,
    pub image_count:  usize,
    pub format:       Format,
    pub present_mode: PresentMode,
}

pub struct Swapchain
{
    pub(crate) device:      Shared<RawDevice>,
    pub(crate) raw:         VkSwapchainKHR,
    pub(crate) images:      SmallArray8<VkImage>,
    pub(crate) image_views: SmallArray8<VkImageView>,
    pub(crate) format:      VkFormat,
}

impl Swapchain
{
    pub fn acquire_image(
        &mut self,
        fence: Option<Fence>,
        sem: Option<Semaphore>,
    ) -> Result<u32, Error>
    {
        self.device
            .device
            .acquire_next_image_khr(
                self.raw,
                core::u64::MAX,
                sem.unwrap_or(VkSemaphore::null()),
                fence.unwrap_or(VkFence::null()),
            )
            .map(|(_, i)| i)
            .map_err(|e| Error::Vulkan(e))
    }

    pub fn present<C>(
        &mut self,
        queue: &Queue<C>,
        index: u32,
        wait_sems: &[Semaphore],
    ) -> Result<(), Error>
    where
        C: capabilities::QueueType,
    {
        let swapchains = [self.raw; 1];
        let indices = [index; 1];

        let present_info = VkPresentInfoKHRBuilder::new()
            .p_wait_semaphores(wait_sems)
            .p_swapchains(&swapchains)
            .p_image_indices(&indices);

        self.device
            .device
            .queue_present_khr(queue.raw(), &present_info)
            .map(|_| ())
            .map_err(|e| Error::Vulkan(e))
    }

    pub fn get_image(&self, index: u32) -> Option<Image>
    {
        if (index as usize) < self.images.len()
        {
            Some(self.images[index as usize])
        }
        else
        {
            None
        }
    }
}
