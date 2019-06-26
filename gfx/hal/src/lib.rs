pub mod capabilities;
mod device;
mod instance;
mod queue;
mod surface;

pub use device::*;
pub use instance::*;
pub use queue::*;
pub use surface::*;

pub trait Backend: Sized
{
    type Error: core::fmt::Debug;
    type Instance: Instance<Self>;
    type Surface: Surface<Self>;
    type PhysicalDevice;
    type QueueFamily: QueueFamily;
    type Queue;
    type Device: Device;
}
