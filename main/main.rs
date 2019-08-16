#![feature(proc_macro_hygiene, start, lang_items)]
#![no_std]

#[cfg(target_env = "msvc")]
mod msvc;

use gfx_hal as hal;
use gfx_hal::{self, Device, Instance, QueueFamily, Surface};
use gfx_vulkan_backend as gfx_vk;

use fnd::{sync::Mutex, *};

use tlsf::Tlsf;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize
{
    main();
    return 0;
}

static mut ALLOC: Option<Mutex<Tlsf>> = None;

unsafe fn init_global_allocator()
{
    ALLOC = Some(Mutex::new(Tlsf::new()));
    fnd::alloc::set_global_allocator(ALLOC.as_mut().unwrap());
}

fn main()
{
    unsafe {
        init_global_allocator();
    }

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

    let mut command_pool = device
        .create_command_pool(&queue, hal::CommandPoolFlags::default())
        .unwrap();

    let mut cmd_buffer = command_pool.alloc_primary_command_buffer().unwrap();
    cmd_buffer.begin().unwrap();
    cmd_buffer.end().unwrap();

    window.events_loop(|event| match *event
    {
        wsi::Event::DestroyWindow => false,
    });

    device.wait_idle();
    command_pool.free_command_buffer(cmd_buffer);
    device.destroy_command_pool(command_pool);
    device.destroy_swapchain(swapchain);
}
