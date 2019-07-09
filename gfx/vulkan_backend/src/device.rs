use super::{conv::*, Backend, Error, RawInstance, Surface, Swapchain};

use fnd::{containers::SmallArray8, Shared};
use gfx_hal as hal;
use vk::{self, builders::*, types::*};

use hal::QueueFamily;

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
    pub(crate) gpu: VkPhysicalDevice,
}

impl Device
{
    fn find_best_image_count(
        &self,
        surface: VkSurfaceKHR,
        preferred_count: u32,
    ) -> Result<u32, VkResult>
    {
        let surface_prps = self
            .instance
            .instance
            .get_physical_device_surface_capabilities_khr(self.gpu, surface)?
            .1;

        return Ok(surface_prps
            .min_image_count
            .max(surface_prps.max_image_count.min(preferred_count)));
    }

    fn find_best_surface_format(
        &self,
        surface: VkSurfaceKHR,
        preferred_format: VkFormat,
        preferred_color_space: VkColorSpaceKHR,
    ) -> Result<(VkFormat, VkColorSpaceKHR), VkResult>
    {
        let format_count = self
            .instance
            .instance
            .get_physical_device_surface_formats_khr_count(self.gpu, surface)?
            .1;

        let mut formats = SmallArray8::new();
        formats.resize_default(format_count);

        self.instance
            .instance
            .get_physical_device_surface_formats_khr(self.gpu, surface, &mut formats)?;

        return if format_count == 1 && formats[0].format == VkFormat::UNDEFINED
        {
            Ok((preferred_format, preferred_color_space))
        }
        else
        {
            Ok(formats
                .iter()
                .filter(|f| f.format == preferred_format && f.color_space == preferred_color_space)
                .nth(0)
                .map(|f| (f.format, f.color_space))
                .unwrap_or((
                    VkFormat::B8G8R8A8_UNORM,
                    VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
                )))
        };
    }

    fn find_best_present_mode(
        &self,
        surface: VkSurfaceKHR,
        preferred_present_mode: VkPresentModeKHR,
    ) -> Result<VkPresentModeKHR, VkResult>
    {
        let present_mode_count = self
            .instance
            .instance
            .get_physical_device_surface_present_modes_khr_count(self.gpu, surface)?
            .1;

        let mut present_modes = SmallArray8::new();
        present_modes.resize_default(present_mode_count);

        self.instance
            .instance
            .get_physical_device_surface_present_modes_khr(self.gpu, surface, &mut present_modes)?;

        return Ok(present_modes
            .iter()
            .copied()
            .filter(|p| *p == preferred_present_mode)
            .nth(0)
            .unwrap_or(VkPresentModeKHR::FIFO_KHR));
    }
}

impl hal::Device<Backend> for Device
{
    fn create_swapchain(
        &self,
        surface: &Surface,
        config: &hal::SwapchainConfig<Backend>,
    ) -> Result<Swapchain, Error>
    {
        let queue_families = &[config.queue_family.id() as u32];

        let image_count = self
            .find_best_image_count(surface.raw.surface, config.image_count as u32)
            .map_err(|e| Error::Swapchain(e))?;

        let (format, color_space) = self
            .find_best_surface_format(
                surface.raw.surface,
                hal_to_vk_format(config.format),
                VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
            )
            .map_err(|e| Error::Swapchain(e))?;

        let present_mode = self
            .find_best_present_mode(
                surface.raw.surface,
                hal_to_vk_present_mode(config.present_mode),
            )
            .map_err(|e| Error::Swapchain(e))?;

        let extent = self
            .instance
            .instance
            .get_physical_device_surface_capabilities_khr(self.gpu, surface.raw.surface)
            .map_err(|e| Error::Swapchain(e))?
            .1
            .current_extent;

        let create_info = VkSwapchainCreateInfoKHRBuilder::new()
            .surface(surface.raw.surface)
            .min_image_count(image_count)
            .image_format(format)
            .image_color_space(color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(VkImageUsageFlagBits::COLOR_ATTACHMENT_BIT)
            .image_sharing_mode(VkSharingMode::EXCLUSIVE)
            .p_queue_family_indices(queue_families)
            .pre_transform(VkSurfaceTransformFlagBitsKHR::IDENTITY_BIT_KHR)
            .composite_alpha(VkCompositeAlphaFlagBitsKHR::OPAQUE_BIT_KHR)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(VkSwapchainKHR::null());

        let swapchain = self
            .raw
            .device
            .create_swapchain_khr(&create_info, None)
            .map_err(|e| Error::Swapchain(e))?
            .1;

        let image_count = self
            .raw
            .device
            .get_swapchain_images_khr_count(swapchain)
            .map_err(|e| Error::Swapchain(e))?
            .1;

        let mut images = SmallArray8::new();
        images.resize_default(image_count);

        self.raw
            .device
            .get_swapchain_images_khr(swapchain, &mut images)
            .map_err(|e| Error::Swapchain(e))?;

        let create_info = VkImageViewCreateInfoBuilder::new()
            .view_type(VkImageViewType::K_2D)
            .format(format)
            .components(
                VkComponentMappingBuilder::new()
                    .r(VkComponentSwizzle::R)
                    .g(VkComponentSwizzle::G)
                    .b(VkComponentSwizzle::B)
                    .a(VkComponentSwizzle::A)
                    .build(),
            )
            .subresource_range(
                VkImageSubresourceRangeBuilder::new()
                    .aspect_mask(VkImageAspectFlags::COLOR_BIT)
                    .base_mip_level(0)
                    .base_array_layer(0)
                    .level_count(1)
                    .layer_count(1)
                    .build(),
            );

        let mut image_views: SmallArray8<VkImageView> = SmallArray8::new();
        for img in images.iter()
        {
            let create_info = create_info.clone().image(*img);
            let view = self
                .raw
                .device
                .create_image_view(&create_info, None)
                .map_err(|e| Error::Swapchain(e))?
                .1;
            image_views.push(view);
        }

        Ok(Swapchain {
            raw: swapchain,
            images,
            image_views,
            format,
        })
    }

    fn destroy_swapchain(&self, swapchain: Swapchain)
    {
        for view in swapchain.image_views.iter()
        {
            self.raw.device.destroy_image_view(*view, None);
        }

        self.raw.device.destroy_swapchain_khr(swapchain.raw, None);
    }

    fn create_command_pool<C>(
        &self,
        queue: &hal::Queue<Backend, C>,
        flags: hal::CommandPoolFlags,
    ) -> Result<VkCommandPool, Error>
    where
        C: hal::capabilities::Capability,
    {
        let flags = hal_to_vk_command_pool_flags(flags);

        let create_info = VkCommandPoolCreateInfoBuilder::new()
            .flags(flags)
            .queue_family_index(queue.id() as u32);

        self.raw
            .device
            .create_command_pool(&create_info, None)
            .map_err(|r| Error::CommandPool(r))
            .map(|(_, p)| p)
    }

    fn destroy_command_pool(&self, pool: VkCommandPool)
    {
        self.raw.device.destroy_command_pool(pool, None);
    }

    fn wait_idle(&self)
    {
        core::mem::drop(self.raw.device.device_wait_idle());
    }
}
