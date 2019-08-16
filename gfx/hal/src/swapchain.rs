use super::{capabilities, Backend, Format, Queue};

#[derive(Copy, Clone)]
pub enum PresentMode
{
    Immediate,
    Mailbox,
    Fifo,
    Relaxed,
}

pub struct SwapchainConfig<'a, B: Backend>
{
    pub queue_family: &'a B::QueueFamilyGroup,
    pub image_count: usize,
    pub format: Format,
    pub present_mode: PresentMode,
}

pub trait Swapchain<B: Backend>
{
    fn acquire_image(
        &mut self,
        fence: Option<B::Fence>,
        sem: Option<B::Semaphore>,
    ) -> Result<u32, B::Error>;

    fn present<C>(
        &mut self,
        queue: &Queue<B, C>,
        index: u32,
        wait_sems: &[B::Semaphore],
    ) -> Result<(), B::Error>
    where
        C: capabilities::QueueType;

    fn get_image(&self, index: u32) -> Option<&B::Image>;
}
