use fnd::containers::Array;
use vk::types::*;

pub struct Swapchain
{
    pub(crate) raw: VkSwapchainKHR,
    pub(crate) images: Array<VkImage>,
    pub(crate) image_views: Array<VkImageView>,
    pub(crate) format: VkFormat,
}
