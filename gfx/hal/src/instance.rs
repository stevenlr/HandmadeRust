use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{SmallArray8, String},
};

use super::{Backend, CreatedDevice};

pub trait Instance<B: Backend>
{
    fn enumerate_gpus_with<A: Allocator + Clone>(
        &self,
        a: A,
    ) -> Result<SmallArray8<Gpu<B, A>, A>, B::Error>;

    fn enumerate_gpus(&self) -> Result<SmallArray8<Gpu<B>>, B::Error>
    {
        self.enumerate_gpus_with(GlobalAllocator)
    }

    fn create_device<A: Allocator>(
        &self,
        gpu: &Gpu<B, A>,
        queues: &[(&B::QueueFamily, &[f32])],
    ) -> Result<CreatedDevice<B>, B::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub queue_families: SmallArray8<B::QueueFamily, A>,
}
