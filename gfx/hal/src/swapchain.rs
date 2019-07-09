use super::{Backend, Format};

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
