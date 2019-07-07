use fnd::containers::SmallArray8;
use vk::types::*;

pub struct Swapchain
{
    pub(crate) raw: VkSwapchainKHR,
    pub(crate) images: SmallArray8<VkImage>,
    pub(crate) image_views: SmallArray8<VkImageView>,
    pub(crate) format: VkFormat,
}
