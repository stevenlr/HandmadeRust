use crate::{Backend, Error, RawDevice};

use fnd::{containers::SmallArray8, Shared};
use gfx_hal as hal;
use vk::{builders::*, types::*};

pub struct Swapchain
{
    pub(crate) device: Shared<RawDevice>,
    pub(crate) raw: VkSwapchainKHR,
    pub(crate) images: SmallArray8<VkImage>,
    pub(crate) image_views: SmallArray8<VkImageView>,
    pub(crate) format: VkFormat,
}

impl hal::Swapchain<Backend> for Swapchain
{
    fn acquire_image(
        &mut self,
        fence: Option<VkFence>,
        sem: Option<VkSemaphore>,
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
            .map_err(|e| Error::Swapchain(e))
    }

    fn present<C>(
        &mut self,
        queue: &hal::Queue<Backend, C>,
        index: u32,
        wait_sems: &[VkSemaphore],
    ) -> Result<(), Error>
    where
        C: hal::capabilities::QueueType,
    {
        let swapchains = [self.raw; 1];
        let indices = [index; 1];

        let present_info = VkPresentInfoKHRBuilder::new()
            .p_wait_semaphores(wait_sems)
            .p_swapchains(&swapchains)
            .p_image_indices(&indices);

        self.device
            .device
            .queue_present_khr(queue.inner().queue, &present_info)
            .map(|_| ())
            .map_err(|e| Error::Swapchain(e))
    }

    fn get_image(&self, index: u32) -> Option<&VkImage>
    {
        if (index as usize) < self.images.len()
        {
            Some(&self.images[index as usize])
        }
        else
        {
            None
        }
    }
}
