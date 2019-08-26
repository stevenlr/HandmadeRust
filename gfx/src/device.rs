use super::{capabilities, conv::*, *};

use core::marker::PhantomData;
use fnd::{bitflags, containers::SmallArray8, Shared};
use vk::{self, builders::*, types::*};

bitflags! {
    pub enum CommandPoolFlags: u8 {
        TRANSIENT = 1,
        RESET_COMMAND_BUFFER = 2,
    }
}

pub struct CreatedDevice
{
    pub device: Option<Device>,
    pub queues: SmallArray8<Option<InnerQueue>>,
}

impl CreatedDevice
{
    pub fn retrieve_device(&mut self) -> Result<Device, QueueRetrievalError>
    {
        self.device
            .take()
            .ok_or(QueueRetrievalError::AlreadyRetrieved)
    }

    pub fn retrieve_queue<C>(&mut self, index: usize) -> Result<Queue<C>, QueueRetrievalError>
    where
        C: capabilities::QueueType,
    {
        if index >= self.queues.len()
        {
            return Err(QueueRetrievalError::QueueIndexOutOfBounds);
        }

        let queue_slot = &mut self.queues[index];
        if queue_slot.is_some()
        {
            if C::supported_by(queue_slot.as_ref().unwrap().queue_type)
            {
                queue_slot
                    .take()
                    .map(|q| Queue {
                        inner:       q,
                        _capability: PhantomData,
                    })
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

#[derive(Debug)]
pub enum QueueRetrievalError
{
    QueueIndexOutOfBounds,
    AlreadyRetrieved,
    IncompatibleCapabilities,
}

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
    pub(crate) raw:      Shared<RawDevice>,
    pub(crate) instance: Shared<RawInstance>,
    pub(crate) gpu:      VkPhysicalDevice,
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

    pub fn create_swapchain(
        &self,
        surface: &Surface,
        config: &SwapchainConfig,
    ) -> Result<Swapchain, Error>
    {
        let queue_families = &[config.queue_family.id as u32];

        let image_count = self
            .find_best_image_count(surface.raw.surface, config.image_count as u32)
            .map_err(|e| Error::Vulkan(e))?;

        let (format, color_space) = self
            .find_best_surface_format(
                surface.raw.surface,
                hal_to_vk_format(config.format),
                VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
            )
            .map_err(|e| Error::Vulkan(e))?;

        let present_mode = self
            .find_best_present_mode(
                surface.raw.surface,
                hal_to_vk_present_mode(config.present_mode),
            )
            .map_err(|e| Error::Vulkan(e))?;

        let extent = self
            .instance
            .instance
            .get_physical_device_surface_capabilities_khr(self.gpu, surface.raw.surface)
            .map_err(|e| Error::Vulkan(e))?
            .1
            .current_extent;

        let create_info = VkSwapchainCreateInfoKHRBuilder::new()
            .surface(surface.raw.surface)
            .min_image_count(image_count)
            .image_format(format)
            .image_color_space(color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(
                VkImageUsageFlagBits::COLOR_ATTACHMENT_BIT | VkImageUsageFlagBits::TRANSFER_DST_BIT,
            )
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
            .map_err(|e| Error::Vulkan(e))?
            .1;

        let image_count = self
            .raw
            .device
            .get_swapchain_images_khr_count(swapchain)
            .map_err(|e| Error::Vulkan(e))?
            .1;

        let mut images = SmallArray8::new();
        images.resize_default(image_count);

        self.raw
            .device
            .get_swapchain_images_khr(swapchain, &mut images)
            .map_err(|e| Error::Vulkan(e))?;

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
                .map_err(|e| Error::Vulkan(e))?
                .1;
            image_views.push(view);
        }

        Ok(Swapchain {
            device: self.raw.clone(),
            raw: swapchain,
            images,
            image_views,
            format,
        })
    }

    pub fn destroy_swapchain(&self, swapchain: Swapchain)
    {
        for view in swapchain.image_views.iter()
        {
            self.raw.device.destroy_image_view(*view, None);
        }

        self.raw.device.destroy_swapchain_khr(swapchain.raw, None);
    }

    pub fn create_command_pool<C>(
        &self,
        queue: &Queue<C>,
        flags: CommandPoolFlags,
    ) -> Result<CommandPool<C>, Error>
    where
        C: capabilities::QueueType,
    {
        let flags = hal_to_vk_command_pool_flags(flags);

        let create_info = VkCommandPoolCreateInfoBuilder::new()
            .flags(flags)
            .queue_family_index(queue.inner.family_index as u32);

        self.raw
            .device
            .create_command_pool(&create_info, None)
            .map_err(|r| Error::Vulkan(r))
            .map(|(_, p)| {
                CommandPool::new(InnerCommandPool {
                    raw:    p,
                    device: self.raw.clone(),
                })
            })
    }

    pub fn destroy_command_pool<C>(&self, pool: CommandPool<C>)
    where
        C: capabilities::QueueType,
    {
        self.raw.device.destroy_command_pool(pool.inner.raw, None);
    }

    pub fn wait_idle(&self)
    {
        drop(self.raw.device.device_wait_idle());
    }

    pub fn create_fence(&self) -> Result<Fence, Error>
    {
        let create_info = VkFenceCreateInfoBuilder::new();

        self.raw
            .device
            .create_fence(&create_info, None)
            .map(|(_, f)| f)
            .map_err(|e| Error::Vulkan(e))
    }

    pub fn destroy_fence(&self, fence: Fence)
    {
        self.raw.device.destroy_fence(fence, None);
    }

    pub fn get_fence_status(&self, fence: Fence) -> Result<bool, Error>
    {
        self.raw
            .device
            .get_fence_status(fence)
            .map(|e| e == VkResult::SUCCESS)
            .map_err(|e| Error::Vulkan(e))
    }

    pub fn reset_fence(&self, fence: Fence)
    {
        drop(self.raw.device.reset_fences(&[fence]));
    }

    pub fn create_semaphore(&self) -> Result<Semaphore, Error>
    {
        let create_info = VkSemaphoreCreateInfoBuilder::new();

        self.raw
            .device
            .create_semaphore(&create_info, None)
            .map(|(_, f)| f)
            .map_err(|e| Error::Vulkan(e))
    }

    pub fn destroy_semaphore(&self, sem: Semaphore)
    {
        self.raw.device.destroy_semaphore(sem, None);
    }
}
