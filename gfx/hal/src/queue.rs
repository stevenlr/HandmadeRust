#[derive(Clone, Copy, Debug)]
pub enum QueueType
{
    Graphics,
    Compute,
    Transfer,
    General,
}

pub trait QueueFamily
{
    fn queue_type(&self) -> QueueType;
    fn id(&self) -> usize;
    fn count(&self) -> usize;

    fn supports_graphics(&self) -> bool
    {
        match self.queue_type()
        {
            QueueType::Graphics | QueueType::General => true,
            QueueType::Compute | QueueType::Transfer => false,
        }
    }

    fn supports_compute(&self) -> bool
    {
        match self.queue_type()
        {
            QueueType::Compute | QueueType::General => true,
            QueueType::Graphics | QueueType::Transfer => false,
        }
    }

    fn supports_transfer(&self) -> bool
    {
        match self.queue_type()
        {
            QueueType::Compute | QueueType::General | QueueType::Graphics | QueueType::Transfer =>
            {
                true
            }
        }
    }
}
