use super::Backend;

pub trait Surface<B: Backend>
{
    fn supports_queue_family(&self, queue_family: &B::QueueFamilyGroup) -> bool;
}
