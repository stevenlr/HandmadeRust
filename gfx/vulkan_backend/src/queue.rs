use super::hal;

pub struct QueueFamily
{
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
