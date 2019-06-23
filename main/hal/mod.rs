use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{Array, String},
};

pub mod vulkan;

pub trait Backend: Sized
{
    type Error: core::fmt::Debug;
    type Instance: Instance<Self>;
    type PhysicalDevice;
    type QueueFamily: QueueFamily;
    type Device: Device;
}

pub trait Instance<B: Backend>
{
    fn enumerate_gpus_with<A: Allocator + Clone>(
        &self,
        a: A,
    ) -> Result<Array<Gpu<B, A>, A>, B::Error>;

    fn enumerate_gpus(&self) -> Result<Array<Gpu<B>>, B::Error>
    {
        self.enumerate_gpus_with(GlobalAllocator)
    }

    fn create_device(
        &self,
        gpu: B::PhysicalDevice,
        queues: &[(&B::QueueFamily, &[f32])],
    ) -> Result<B::Device, B::Error>;
}

#[derive(Clone, Copy, Debug)]
pub enum GpuType
{
    DiscreteGpu,
    IntegratedGpu,
    VirtualGpu,
    Cpu,
    Unknown,
}

pub struct Gpu<B: Backend, A: Allocator = GlobalAllocator>
{
    pub name: String,
    pub gpu_type: GpuType,
    pub physical_device: B::PhysicalDevice,
    pub queue_families: Array<B::QueueFamily, A>,
}

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

pub trait Device
{
}
