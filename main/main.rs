#![feature(proc_macro_hygiene, start, lang_items, core_intrinsics)]
#![no_std]

#[cfg(target_env = "msvc")]
mod msvc;

use gfx_hal as hal;
use gfx_hal::{self, Device, Instance, QueueFamily, Surface, Swapchain};
use gfx_vulkan_backend as gfx_vk;

use fnd::{
    containers::{Array, Queue},
    sync::Mutex,
    *,
};

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

// @Todo Implement cos & stuff in fnd
#[inline]
fn cos_f32(f: f32) -> f32
{
    unsafe { core::intrinsics::cosf32(f) }
}

fn color(f: f32) -> (f32, f32, f32)
{
    let r = 0.5 + 0.5 * cos_f32(2.0 * 3.1415 * (1.0 * f + 0.00));
    let g = 0.5 + 0.5 * cos_f32(2.0 * 3.1415 * (1.0 * f + 0.33));
    let b = 0.5 + 0.5 * cos_f32(2.0 * 3.1415 * (1.0 * f + 0.67));
    return (r, g, b);
}

struct ToRelease<B: hal::Backend>
{
    fence:  B::Fence,
    pool:   hal::CommandPool<B, hal::capabilities::General>,
    buffer: hal::CommandBuffer<B, hal::capabilities::General, hal::capabilities::Primary>,
}

struct ResourceTracker<B: hal::Backend>
{
    to_release: Queue<ToRelease<B>>,
    fences:     Array<B::Fence>,
    pools:      Array<hal::CommandPool<B, hal::capabilities::General>>,
}

impl<B: hal::Backend> ResourceTracker<B>
{
    fn new() -> Self
    {
        Self {
            to_release: Queue::new(),
            fences:     Array::new(),
            pools:      Array::new(),
        }
    }

    fn acquire_pool(
        &mut self,
        device: &B::Device,
        queue: &hal::Queue<B, hal::capabilities::General>,
    ) -> hal::CommandPool<B, hal::capabilities::General>
    {
        self.pools.pop().unwrap_or_else(|| {
            device
                .create_command_pool(queue, hal::CommandPoolFlags::default())
                .unwrap()
        })
    }

    fn acquire_fence(&mut self, device: &B::Device) -> B::Fence
    {
        self.fences
            .pop()
            .unwrap_or_else(|| device.create_fence().unwrap())
    }

    fn push(
        &mut self,
        fence: B::Fence,
        pool: hal::CommandPool<B, hal::capabilities::General>,
        buffer: hal::CommandBuffer<B, hal::capabilities::General, hal::capabilities::Primary>,
    )
    {
        self.to_release.push(ToRelease {
            fence,
            pool,
            buffer,
        });
    }

    fn recycle(&mut self, device: &B::Device)
    {
        while let Some(to_release_ref) = self.to_release.peek()
        {
            let status = device.get_fence_status(to_release_ref.fence).unwrap();
            if !status
            {
                break;
            }

            let ToRelease {
                fence,
                mut pool,
                buffer,
            } = self.to_release.pop().unwrap();

            pool.free_command_buffer(buffer);
            device.reset_fence(fence);
            pool.reset(true);

            self.fences.push(fence);
            self.pools.push(pool);
        }
    }

    fn cleanup(mut self, device: &B::Device)
    {
        device.wait_idle();

        self.recycle(device);

        for fence in self.fences
        {
            device.destroy_fence(fence);
        }

        for pool in self.pools
        {
            device.destroy_command_pool(pool);
        }
    }
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
    let mut queue = created_device
        .retrieve_queue::<hal::capabilities::General>(0)
        .unwrap();

    let mut swapchain = device
        .create_swapchain(
            &surface,
            &hal::SwapchainConfig {
                format: hal::Format::Bgr8Unorm,
                image_count: 3,
                present_mode: hal::PresentMode::Mailbox,
                queue_family,
            },
        )
        .unwrap();

    let mut resource_tracker = ResourceTracker::new();

    let sem_acquire = device.create_semaphore().unwrap();
    let sem_present = device.create_semaphore().unwrap();
    let mut t = 0.0;

    window.events_loop(
        |event| match *event
        {
            wsi::Event::DestroyWindow => false,
        },
        || {
            let mut command_pool = resource_tracker.acquire_pool(&device, &queue);

            let img_index = swapchain.acquire_image(None, Some(sem_acquire)).unwrap();
            let img = swapchain.get_image(img_index).unwrap();

            let mut cmd_buffer = command_pool.alloc_primary_command_buffer().unwrap();
            cmd_buffer.begin().unwrap();

            cmd_buffer.cmd_pipeline_barrier(
                hal::PipelineStageMask::TOP_OF_PIPE..hal::PipelineStageMask::TRANSFER,
                &[hal::Barrier::Image {
                    access: hal::AccessMask::default()..hal::AccessMask::TRANSFER_WRITE,
                    layout: hal::ImageLayout::Undefined..hal::ImageLayout::General,
                    queues: None,
                    image:  img,
                    range:  hal::ImageSubresourceRange {
                        aspects:    hal::ImageAspectMask::COLOR,
                        mip_levels: 0..0,
                        layers:     0..0,
                    },
                }],
            );

            let (r, g, b) = color(t);

            cmd_buffer.cmd_clear_color_image(
                img,
                hal::ImageLayout::General,
                hal::ClearColor::Float([r, g, b, 1.0]),
                hal::ImageSubresourceRange {
                    aspects:    hal::ImageAspectMask::COLOR,
                    mip_levels: 0..0,
                    layers:     0..0,
                },
            );

            cmd_buffer.cmd_pipeline_barrier(
                hal::PipelineStageMask::TRANSFER..hal::PipelineStageMask::BOTTOM_OF_PIPE,
                &[hal::Barrier::Image {
                    access: hal::AccessMask::TRANSFER_WRITE..hal::AccessMask::MEMORY_READ,
                    layout: hal::ImageLayout::General..hal::ImageLayout::PresentSrc,
                    queues: None,
                    image:  img,
                    range:  hal::ImageSubresourceRange {
                        aspects:    hal::ImageAspectMask::COLOR,
                        mip_levels: 0..0,
                        layers:     0..0,
                    },
                }],
            );

            cmd_buffer.end().unwrap();

            let fence = resource_tracker.acquire_fence(&device);

            queue
                .submit(
                    &[(sem_acquire, hal::PipelineStageMask::TOP_OF_PIPE)],
                    &[&cmd_buffer],
                    &[sem_present],
                    Some(fence),
                )
                .unwrap();

            swapchain
                .present(&queue, img_index, &[sem_present])
                .unwrap();

            resource_tracker.push(fence, command_pool, cmd_buffer);
            resource_tracker.recycle(&device);

            t += 0.0003;
        },
    );

    device.wait_idle();

    resource_tracker.cleanup(&device);

    device.destroy_semaphore(sem_acquire);
    device.destroy_semaphore(sem_present);
    device.destroy_swapchain(swapchain);
}
