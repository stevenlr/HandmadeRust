use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{Array, String},
};

pub mod vulkan;

pub trait Backend: Sized
{
    type Instance: Instance<Self>;
    type PhysicalDevice;
    type Error: core::fmt::Debug;
}

#[derive(Debug)]
pub enum GpuType
{
    DiscreteGpu,
    IntegratedGpu,
    VirtualGpu,
    Cpu,
    Unknown,
}

pub struct Gpu<B: Backend>
{
    pub name: String,
    pub gpu_type: GpuType,
    pub physical_device: B::PhysicalDevice,
}

pub trait Instance<B: Backend>
{
    fn enumerate_gpus_with<A: Allocator>(&self, a: A) -> Result<Array<Gpu<B>, A>, B::Error>;

    fn enumerate_gpus(&self) -> Result<Array<Gpu<B>, GlobalAllocator>, B::Error>
    {
        self.enumerate_gpus_with(GlobalAllocator)
    }
}
