use fnd::{
    alloc::{Allocator, GlobalAllocator},
    containers::{Array, String},
};

use super::{Backend, CreatedDevice};

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
    ) -> Result<CreatedDevice<B>, B::Error>;
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
