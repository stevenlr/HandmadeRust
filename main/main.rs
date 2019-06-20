#![feature(proc_macro_hygiene)]

mod vk_init;
mod wsi;

use core::mem::transmute;
use fnd::containers::Array;
use vk::types::*;
use vk_init::{init_vulkan, InstanceBuilder, QueueConfig};
use win32::kernel32;
use wsi::Window;

fn find_best_image_count(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    vk_surface: VkSurfaceKHR,
    preferred_count: u32,
) -> Result<u32, VkResult>
{
    let surface_prps = vk_instance
        .get_physical_device_surface_capabilities_khr(gpu, vk_surface)?
        .1;

    return Ok(surface_prps
        .min_image_count
        .max(surface_prps.max_image_count.min(preferred_count)));
}

fn find_best_surface_format(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    vk_surface: VkSurfaceKHR,
    preferred_format: VkFormat,
    preferred_color_space: VkColorSpaceKHR,
) -> Result<(VkFormat, VkColorSpaceKHR), VkResult>
{
    let format_count = vk_instance
        .get_physical_device_surface_formats_khr_count(gpu, vk_surface)?
        .1;

    let mut formats = Array::new();
    formats.resize_default(format_count);

    vk_instance.get_physical_device_surface_formats_khr(gpu, vk_surface, &mut formats)?;

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
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    vk_surface: VkSurfaceKHR,
    preferred_present_mode: VkPresentModeKHR,
) -> Result<VkPresentModeKHR, VkResult>
{
    let present_mode_count = vk_instance
        .get_physical_device_surface_present_modes_khr_count(gpu, vk_surface)?
        .1;
    let mut present_modes = Array::new();
    present_modes.resize_default(present_mode_count);

    vk_instance.get_physical_device_surface_present_modes_khr(
        gpu,
        vk_surface,
        &mut present_modes,
    )?;

    return Ok(present_modes
        .iter()
        .copied()
        .filter(|p| *p == preferred_present_mode)
        .nth(0)
        .unwrap_or(VkPresentModeKHR::FIFO_KHR));
}

fn main()
{
    let window = Window::new(1280, 720, "Handmade Rust").unwrap();

    let vk_module = unsafe { kernel32::LoadLibraryA(b"vulkan-1.dll\0".as_ptr()) };
    let vk_entry = vk::EntryPoint::new(|fn_name| unsafe {
        transmute(kernel32::GetProcAddress(vk_module, fn_name.as_ptr()))
    });

    let queue_configs = [QueueConfig {
        flags: VkQueueFlags::GRAPHICS_BIT | VkQueueFlags::TRANSFER_BIT | VkQueueFlags::COMPUTE_BIT,
        supports_present: true,
    }];

    init_vulkan(vk_entry)
        .enable_debug()
        .enable_wsi()
        .build()
        .build_surface(&window)
        .choose_gpu(|_, prps: &VkPhysicalDeviceProperties| {
            prps.device_type == VkPhysicalDeviceType::DISCRETE_GPU
        })
        .build_device(&queue_configs);

    /*
    let queue_families = &[queue_family_index];

    let image_count = find_best_image_count(&vk_instance, gpu, vk_surface, 3).unwrap();
    let (format, color_space) = find_best_surface_format(
        &vk_instance,
        gpu,
        vk_surface,
        VkFormat::B8G8R8A8_UNORM,
        VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
    )
    .unwrap();
    let present_mode =
        find_best_present_mode(&vk_instance, gpu, vk_surface, VkPresentModeKHR::MAILBOX_KHR)
            .unwrap();
    let extent = vk_instance
        .get_physical_device_surface_capabilities_khr(gpu, vk_surface)
        .unwrap()
        .1
        .current_extent;

    let create_info = VkSwapchainCreateInfoKHRBuilder::new()
        .surface(vk_surface)
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

    let vk_swapchain = vk_device
        .create_swapchain_khr(&create_info, None)
        .unwrap()
        .1;
    println!("Swapchain created");

    let image_count = vk_device
        .get_swapchain_images_khr_count(vk_swapchain)
        .unwrap()
        .1;
    let mut images = Array::new();
    images.resize_default(image_count);

    vk_device
        .get_swapchain_images_khr(vk_swapchain, &mut images)
        .unwrap();
    println!("    Swapchain uses {} images", images.len());

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

    let image_views: Array<VkImageView> = images
        .iter()
        .map(|img| {
            let create_info = create_info.clone().image(*img);
            return vk_device.create_image_view(&create_info, None).unwrap().1;
        })
        .collect();

    let vk_queue = vk_device.get_device_queue(queue_family_index, 0);

    window.events_loop(|e| match *e
    {
        Event::DestroyWindow => false,
    });

    vk_device.queue_wait_idle(vk_queue).unwrap();
    vk_device.device_wait_idle().unwrap();

    image_views.into_iter().for_each(|view| {
        vk_device.destroy_image_view(view, None);
    });

    vk_device.destroy_swapchain_khr(vk_swapchain, None);
    vk_device.destroy_device(None);
    vk_instance.destroy_surface_khr(vk_surface, None);
    vk_instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
    vk_instance.destroy_instance(None);
    */
}
