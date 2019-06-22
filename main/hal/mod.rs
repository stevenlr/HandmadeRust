use fnd::containers::String;

pub mod vulkan;

pub trait Backend: Sized
{
    type Instance: Instance<Self>;
    type PhysicalDevice;
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
    fn enumerate_gpus(&self) -> &[Gpu<B>];
}
