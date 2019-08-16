#![no_std]

mod command_buffer;
mod command_pool;
mod conv;
mod device;
mod instance;
mod queue;
mod surface;
mod swapchain;

use gfx_hal as hal;

pub use command_buffer::*;
pub use command_pool::*;
pub use device::*;
pub use instance::*;
pub use queue::*;
pub use surface::*;
pub use swapchain::*;

use vk::types::*;

pub struct Backend;

impl hal::Backend for Backend
{
    type Error = Error;
    type Instance = Instance;
    type Surface = Surface;
    type PhysicalDevice = VkPhysicalDevice;
    type QueueFamilyGroup = QueueFamilyGroup;
    type InnerQueue = Queue;
    type Device = Device;
    type Fence = VkFence;
    type Semaphore = VkSemaphore;
    type Swapchain = Swapchain;
    type InnerCommandPool = CommandPool;
    type InnerCommandBuffer = CommandBuffer;
    type Image = VkImage;
}

#[derive(Debug)]
pub enum Error
{
    Instance(InstanceError),
    VulkanError(VkResult),
    Surface(VkResult),
    Swapchain(VkResult),
    CommandPool(VkResult),
}
