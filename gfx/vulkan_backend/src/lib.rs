mod conv;
mod device;
mod instance;
mod queue;

use gfx_hal as hal;

pub use device::*;
pub use instance::*;
pub use queue::*;

use vk::types::*;

pub struct Backend;

impl hal::Backend for Backend
{
    type Error = Error;
    type Instance = Instance;
    type PhysicalDevice = VkPhysicalDevice;
    type QueueFamily = QueueFamily;
    type Queue = VkQueue;
    type Device = Device;
}

#[derive(Debug)]
pub enum Error
{
    Instance(InstanceError),
    VulkanError(VkResult),
}
