use vk::types::{VkPhysicalDevice, VkQueue};

use gfx_hal as hal;

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
