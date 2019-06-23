#![feature(proc_macro_hygiene)]
#![allow(unused)] // @Todo Remove this

mod hal;
mod vk_init;
mod wsi;

use fnd::alloc::GlobalAllocator;
/*use core::mem::transmute;
use fnd::dl::DynamicLibrary;
use vk::types::*;
use vk_init::{init_vulkan, InstanceBuilder, QueueConfig, SwapchainParams};
use wsi::{Event, Window};*/

use hal::Instance;

fn main()
{
    let instance = hal::vulkan::Instance::create().unwrap();

    let gpus = instance.enumerate_gpus().unwrap();
    for (index, gpu) in gpus.iter().enumerate()
    {
        println!("[{}] {}: {:?}", index, gpu.name, gpu.gpu_type);
    }

    let gpu = &gpus[0];
    let device = instance
        .create_device(gpus[0].physical_device, &[(&gpu.queue_families[0], &[1.0])])
        .unwrap();
}

fn main2()
{
    /*let window = Window::new(1280, 720, "Handmade Rust").expect("Cannot create window");

    let vk_module =
        DynamicLibrary::load("vulkan-1.dll").expect("Cannot load Vulkan dynamic library");

    let vk_entry = vk::EntryPoint::new(|fn_name| unsafe {
        vk_module
            .get_symbol_from_bytes_null_terminated(fn_name)
            .map(|f: *mut ()| transmute(f))
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
    });*/
}
