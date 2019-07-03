pub mod capabilities;
mod device;
mod instance;
mod queue;
mod surface;
mod swapchain;

pub use device::*;
pub use instance::*;
pub use queue::*;
pub use surface::*;
pub use swapchain::*;

pub trait Backend: Sized
{
    type Error: core::fmt::Debug;
    type Instance: Instance<Self>;
    type Surface: Surface<Self>;
    type PhysicalDevice;
    type QueueFamily: QueueFamily;
    type Queue;
    type Device: Device<Self>;
    type Swapchain;
}

#[derive(Copy, Clone)]
pub enum Format
{
    Rgb8Unorm,
    Rgb8Snorm,
    Rgb8Srgb,
    Bgr8Unorm,
    Bgr8Snorm,
    Bgr8Srgb,
    Rgba8Unorm,
    Rgba8Snorm,
    Rgba8Srgb,
    Bgra8Unorm,
    Bgra8Snorm,
    Bgra8Srgb,
}
