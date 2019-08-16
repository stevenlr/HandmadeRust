#![no_std]

pub mod capabilities;
mod command_buffer;
mod command_pool;
mod device;
mod instance;
mod queue;
mod surface;
mod swapchain;

pub use command_buffer::*;
pub use command_pool::*;
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
    type QueueFamilyGroup: QueueFamilyGroup;
    type InnerQueue: QueueFamily;
    type Device: Device<Self>;
    type Semaphore: Copy;
    type Fence: Copy;
    type Swapchain: Swapchain<Self>;
    type InnerCommandPool: InnerCommandPool<Self>;
    type InnerCommandBuffer: InnerCommandBuffer<Self>;
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
