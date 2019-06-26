#![feature(proc_macro_hygiene)]

use gfx_hal as hal;
use gfx_hal::{self, Instance, QueueFamily, Surface};
use gfx_vulkan_backend as gfx_vk;

fn main()
{
    let window = wsi::Window::new(1280, 720, "Handmade Rust").expect("Window creation");

    let instance = gfx_vk::Instance::create().unwrap();
    let surface = instance.create_surface(&window).expect("Surface creation");

    let gpus = instance.enumerate_gpus().unwrap();
    for (index, gpu) in gpus.iter().enumerate()
    {
        println!("[{}] {}: {:?}", index, gpu.name, gpu.gpu_type);
    }

    let (gpu, queue_family) = gpus
        .iter()
        .filter(|gpu| gpu.gpu_type == hal::GpuType::DiscreteGpu)
        .filter_map(|gpu| {
            gpu.queue_families
                .iter()
                .filter(|q| q.supports_graphics())
                .filter(|q| surface.supports_queue_family(q))
                .map(|q| (gpu, q))
                .nth(0)
        })
        .nth(0)
        .expect("No suitable GPU found");

    let mut created_device = instance
        .create_device(gpu, &[(queue_family, &[1.0])])
        .unwrap();

    let _device = created_device.retrieve_device().unwrap();
    let _queue = created_device
        .retrieve_queue::<gfx_hal::capabilities::General>(0)
        .unwrap();

    window.events_loop(|event| match *event
    {
        wsi::Event::DestroyWindow => false,
    });
}

/*
let swapchain_params = SwapchainParams {
    queue_index: 0,
    image_count: 3,
    format: VkFormat::B8G8R8A8_UNORM,
    color_space: VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
    present_mode: VkPresentModeKHR::MAILBOX_KHR,
};

vk_ctx.init_swapchain(&swapchain_params);
*/
