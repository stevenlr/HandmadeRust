use super::{hal, RawInstance};

use fnd::Shared;
use vk;

pub(crate) struct RawDevice
{
    pub(crate) device: vk::Device,
}

impl Drop for RawDevice
{
    fn drop(&mut self)
    {
        self.device.destroy_device(None);
    }
}

pub struct Device
{
    pub(crate) raw: Shared<RawDevice>,
    pub(crate) instance: Shared<RawInstance>,
}

impl hal::Device for Device {}
