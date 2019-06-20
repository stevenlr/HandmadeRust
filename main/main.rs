#![feature(proc_macro_hygiene)]

mod vk_init;
mod wsi;

use core::mem::transmute;
use vk::types::*;
use vk_init::{init_vulkan, InstanceBuilder, QueueConfig, SwapchainParams};
use win32::kernel32;
use wsi::{Event, Window};

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

    let mut vk_ctx = init_vulkan(vk_entry)
        .enable_debug()
        .enable_wsi()
        .build()
        .build_surface(&window)
        .choose_gpu(|_, prps: &VkPhysicalDeviceProperties| {
            prps.device_type == VkPhysicalDeviceType::DISCRETE_GPU
        })
        .build_device(&queue_configs);

    let swapchain_params = SwapchainParams {
        queue_index: 0,
        image_count: 3,
        format: VkFormat::B8G8R8A8_UNORM,
        color_space: VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
        present_mode: VkPresentModeKHR::MAILBOX_KHR,
    };

    vk_ctx.init_swapchain(&swapchain_params);

    window.events_loop(|e| match *e
    {
        Event::DestroyWindow => false,
    });
}
