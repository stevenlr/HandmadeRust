mod conv;
mod device;
mod instance;
mod queue;
mod surface;
mod swapchain;

use gfx_hal as hal;

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
    type QueueFamily = QueueFamily;
    type Queue = VkQueue;
    type Device = Device;
    type Swapchain = Swapchain;
}

#[derive(Debug)]
pub enum Error
{
    Instance(InstanceError),
    VulkanError(VkResult),
    Surface(VkResult),
    Swapchain(VkResult),
}
