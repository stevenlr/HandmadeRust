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
    type Queue;
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

#[derive(Clone, Copy, Debug)]
pub enum QueueType
{
    Graphics,
    Compute,
    Transfer,
    General,
}

pub mod capabilities
{
    use super::QueueType;

    pub trait Capability
    {
        fn supported_by(queue_type: QueueType) -> bool;
    }

    pub trait Supports<T: Capability>
    {
    }

    pub struct Compute;
    pub struct Graphics;
    pub struct Transfer;
    pub struct General;

    impl Capability for Compute
    {
        #[inline]
        fn supported_by(queue_type: QueueType) -> bool
        {
            match queue_type
            {
                QueueType::Compute => true,
                QueueType::General => true,
                _ => false,
            }
        }
    }

    impl Capability for Graphics
    {
        #[inline]
        fn supported_by(queue_type: QueueType) -> bool
        {
            match queue_type
            {
                QueueType::General => true,
                QueueType::Graphics => true,
                _ => false,
            }
        }
    }

    impl Capability for Transfer
    {
        #[inline]
        fn supported_by(queue_type: QueueType) -> bool
        {
            match queue_type
            {
                QueueType::Compute => true,
                QueueType::General => true,
                QueueType::Graphics => true,
                QueueType::Transfer => true,
            }
        }
    }

    impl Capability for General
    {
        #[inline]
        fn supported_by(queue_type: QueueType) -> bool
        {
            match queue_type
            {
                QueueType::General => true,
                _ => false,
            }
        }
    }

    impl<T: Capability> Supports<T> for T {}

    impl Supports<Transfer> for General {}
    impl Supports<Compute> for General {}
    impl Supports<Graphics> for General {}

    impl Supports<Transfer> for Graphics {}
    impl Supports<Transfer> for Compute {}
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

pub struct CreatedQueue<B: Backend>
{
    queue_type: QueueType,
    queue: Option<B::Queue>,
}

pub struct CreatedDevice<B: Backend>
{
    device: Option<B::Device>,
    queues: Array<CreatedQueue<B>>,
}

#[derive(Debug)]
pub enum QueueRetrievalError
{
    QueueIndexOutOfBounds,
    AlreadyRetrieved,
    IncompatibleCapabilities,
}

impl<B: Backend> CreatedDevice<B>
{
    pub fn retrieve_device(&mut self) -> Result<B::Device, QueueRetrievalError>
    {
        core::mem::replace(&mut self.device, None).ok_or(QueueRetrievalError::AlreadyRetrieved)
    }

    pub fn retrieve_queue<C>(&mut self, index: usize) -> Result<B::Queue, QueueRetrievalError>
    where
        C: capabilities::Capability,
    {
        if index >= self.queues.len()
        {
            return Err(QueueRetrievalError::QueueIndexOutOfBounds);
        }

        if self.queues[index].queue.is_some()
        {
            if C::supported_by(self.queues[index].queue_type)
            {
                core::mem::replace(&mut self.queues[index].queue, None)
                    .ok_or(QueueRetrievalError::AlreadyRetrieved)
            }
            else
            {
                Err(QueueRetrievalError::IncompatibleCapabilities)
            }
        }
        else
        {
            Err(QueueRetrievalError::AlreadyRetrieved)
        }
    }
}

pub trait Device
{
}
