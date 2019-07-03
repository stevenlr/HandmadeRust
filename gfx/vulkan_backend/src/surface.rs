use fnd::Shared;
use gfx_hal as hal;
use vk::{builders::*, types::*};
use wsi;

use super::{Backend, Error, QueueFamily, RawInstance};

pub(crate) struct RawSurface
{
    pub(crate) surface: VkSurfaceKHR,
    raw_instance: Shared<RawInstance>,
}

pub struct Surface
{
    pub(crate) raw: Shared<RawSurface>,
}

impl Surface
{
    pub(crate) fn create(
        raw_instance: Shared<RawInstance>,
        window: &wsi::Window,
    ) -> Result<Self, Error>
    {
        let create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
            .hinstance(window.win32_hinstance())
            .hwnd(window.win32_hwnd());

        let vk_surface = raw_instance
            .instance
            .create_win_32_surface_khr(&create_info, None)
            .map(|p| p.1)
            .map_err(|e| Error::Surface(e))?;

        Ok(Self {
            raw: Shared::new(RawSurface {
                surface: vk_surface,
                raw_instance,
            }),
        })
    }
}

impl hal::Surface<Backend> for Surface
{
    fn supports_queue_family(&self, queue_family: &QueueFamily) -> bool
    {
        self.raw
            .raw_instance
            .instance
            .get_physical_device_surface_support_khr(
                queue_family.physical_device,
                queue_family.id as u32,
                self.raw.surface,
            )
            .map(|(_, b)| b)
            .unwrap_or(false)
    }
}

impl Drop for RawSurface
{
    fn drop(&mut self)
    {
        self.raw_instance
            .instance
            .destroy_surface_khr(self.surface, None);
    }
}
