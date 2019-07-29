#![feature(proc_macro_hygiene)]

use gfx_hal as hal;
use gfx_hal::{self, Device, Instance, QueueFamily, Surface};
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

    let device = created_device.retrieve_device().unwrap();
    let queue = created_device
        .retrieve_queue::<gfx_hal::capabilities::General>(0)
        .unwrap();

    let swapchain = device
        .create_swapchain(
            &surface,
            &hal::SwapchainConfig {
                format: hal::Format::Bgr8Unorm,
                image_count: 3,
                present_mode: hal::PresentMode::Mailbox,
                queue_family: queue_family,
            },
        )
        .unwrap();

    let command_pool = device
        .create_command_pool(&queue, hal::CommandPoolFlags::default())
        .unwrap();

    window.events_loop(|event| match *event
    {
        wsi::Event::DestroyWindow => false,
    });

    device.wait_idle();
    device.destroy_command_pool(command_pool);
    device.destroy_swapchain(swapchain);
}
