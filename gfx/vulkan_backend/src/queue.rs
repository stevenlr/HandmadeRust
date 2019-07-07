use vk::types::VkPhysicalDevice;

use gfx_hal as hal;

#[derive(Clone, Copy)]
pub struct QueueFamily
{
    pub(crate) physical_device: VkPhysicalDevice,
    pub(crate) queue_type: hal::QueueType,
    pub(crate) id: usize,
    pub(crate) count: usize,
}

impl hal::QueueFamily for QueueFamily
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

    #[inline]
    fn count(&self) -> usize
    {
        self.count
    }
}
