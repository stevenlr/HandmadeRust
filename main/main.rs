use core::{
    cell::RefCell,
    marker::PhantomPinned,
    mem::transmute,
    pin::Pin,
    ptr::{null, null_mut},
};

use fnd::{
    alloc::{set_global_allocator, SystemAllocator},
    containers::{Array, Queue},
    str::CStr,
    Unq,
};

use vk::{builders::*, types::*};

use win32::{kernel32, user32};

static mut ALLOCATOR: Option<&SystemAllocator> = None;

fn init_global_allocator()
{
    let allocator = SystemAllocator::default();
    unsafe {
        ALLOCATOR = Some(transmute(Unq::leak(Unq::new_with(
            SystemAllocator::default(),
            &allocator,
        ))));
        set_global_allocator(ALLOCATOR.as_mut().unwrap());
    }
}

fn get_gpu_queue_family_properties(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
) -> Array<VkQueueFamilyProperties>
{
    let mut prps = Array::new();

    prps.resize(
        vk_instance.get_physical_device_queue_family_properties_count(gpu),
        VkQueueFamilyProperties::default(),
    );
    vk_instance.get_physical_device_queue_family_properties(gpu, &mut prps);

    return prps;
}

enum Event
{
    DestroyWindow,
}

struct EventQueue
{
    queue: RefCell<Queue<Event>>,

    // We add this so EventQueue is !Unpin because we need
    // the raw pointer to the queue in the events callback.
    _pin: PhantomPinned,
}

impl EventQueue
{
    fn new() -> Self
    {
        Self {
            queue: RefCell::new(Queue::new()),
            _pin: PhantomPinned,
        }
    }

    #[inline]
    fn queue_event(&self, event: Event)
    {
        self.queue.borrow_mut().push(event);
    }

    #[inline]
    fn poll_event(&self) -> Option<Event>
    {
        self.queue.borrow_mut().pop()
    }
}

fn queue_event(window: win32::HWND, event: Event)
{
    let queue =
        unsafe { user32::GetWindowLongPtrA(window, win32::GWLP_USERDATA) as *const EventQueue };

    if !queue.is_null()
    {
        let queue: &EventQueue = unsafe { &*queue };
        queue.queue_event(event);
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: win32::HWND,
    msg: win32::UINT,
    w_param: win32::WPARAM,
    l_param: win32::LPARAM,
) -> win32::LRESULT
{
    match msg
    {
        win32::WM_DESTROY =>
        {
            queue_event(hwnd, Event::DestroyWindow);
            0
        }
        _ => user32::DefWindowProcA(hwnd, msg, w_param, l_param),
    }
}

struct Window
{
    hinstance: win32::HINSTANCE,
    window: win32::HWND,
    queue: Pin<Unq<EventQueue>>,
}

impl Window
{
    fn new(width: i32, height: i32, title: &str) -> Option<Self>
    {
        let hinstance = unsafe { kernel32::GetModuleHandleA(0 as _) as win32::HINSTANCE };

        let wnd_class_name = b"HandmadeRustClass\0".as_ptr();

        let wnd_class = win32::WNDCLASSA {
            style: win32::CS_VREDRAW | win32::CS_HREDRAW,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: wnd_class_name,
        };

        // @Todo Make CString
        let mut title_z: Array<_> = title.bytes().collect();
        title_z.push(0);

        let window = unsafe {
            user32::RegisterClassA(&wnd_class);
            user32::CreateWindowExA(
                0,
                wnd_class_name,
                title_z.as_ptr(),
                win32::WS_OVERLAPPEDWINDOW,
                win32::CW_USEDEFAULT,
                win32::CW_USEDEFAULT,
                width,
                height,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut(),
            )
        };

        if window == null_mut()
        {
            None
        }
        else
        {
            let event_queue = Unq::pin(EventQueue::new());

            unsafe {
                user32::ShowWindow(window, win32::SW_SHOW);
                user32::SetWindowLongPtrA(
                    window,
                    win32::GWLP_USERDATA,
                    &*event_queue as *const _ as _,
                );
            }

            Some(Self {
                hinstance,
                window,
                queue: event_queue,
            })
        }
    }

    fn create_vk_surface(&self, vk_instance: &vk::Instance) -> Result<VkSurfaceKHR, VkResult>
    {
        let create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
            .hinstance(self.hinstance)
            .hwnd(self.window);

        vk_instance
            .create_win_32_surface_khr(&create_info, None)
            .map(|p| p.1)
    }

    fn handle_events(&self)
    {
        unsafe {
            let mut msg: win32::MSG = core::mem::zeroed();
            while user32::PeekMessageA(&mut msg, self.window, 0, 0, win32::PM_REMOVE) > 0
            {
                user32::TranslateMessage(&msg);
                user32::DispatchMessageA(&msg);
            }
        }
    }

    #[inline]
    fn poll_event(&self) -> Option<Event>
    {
        self.handle_events();
        self.queue.poll_event()
    }
}

impl Drop for Window
{
    fn drop(&mut self)
    {
        unsafe {
            user32::SetWindowLongPtrA(self.window, win32::GWLP_USERDATA, 0);
            user32::DestroyWindow(self.window);
        }
    }
}

fn main()
{
    init_global_allocator();

    let window = Window::new(1280, 720, "Handmade Rust").unwrap();

    let vk_module = unsafe { kernel32::LoadLibraryA(b"vulkan-1.dll\0".as_ptr()) };
    let vk_entry = vk::EntryPoint::new(|fn_name| unsafe {
        transmute(kernel32::GetProcAddress(vk_module, fn_name.as_ptr()))
    });

    let instance_extensions = &[
        //VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_SURFACE_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C.as_ptr(),
    ];

    let layers = &[b"VK_LAYER_LUNARG_standard_validation\0".as_ptr()];

    let create_info = VkInstanceCreateInfoBuilder::new()
        .pp_enabled_extension_names(instance_extensions)
        .pp_enabled_layer_names(layers);

    let vk_instance = vk_entry.create_instance(&create_info, None).unwrap().1;
    let vk_instance = vk::Instance::new(vk_instance, &vk_entry);

    let gpu_count = vk_instance.enumerate_physical_devices_count().unwrap().1;
    println!("{} GPU(s)", gpu_count);

    let gpus = {
        let mut gpus = Array::new();
        gpus.resize(gpu_count, VkPhysicalDevice::null());
        vk_instance.enumerate_physical_devices(&mut gpus).unwrap();
        gpus
    };

    for (index, gpu) in gpus.iter().enumerate()
    {
        let prps = vk_instance.get_physical_device_properties(*gpu);
        let name = unsafe { CStr::from_bytes_null_terminated_unchecked(prps.device_name.as_ptr()) };
        println!("    {}: {}", index, name.as_str().unwrap());
    }

    let vk_surface = window.create_vk_surface(&vk_instance).unwrap();

    let gpu = *gpus.iter().nth(0).unwrap();
    println!("Using GPU 0");

    let queue_family_properties = get_gpu_queue_family_properties(&vk_instance, gpu);
    let queue_family_index = queue_family_properties
        .iter()
        .enumerate()
        .filter(|(_, prps)| {
            prps.queue_flags.contains(
                VkQueueFlagBits::GRAPHICS_BIT
                    | VkQueueFlagBits::COMPUTE_BIT
                    | VkQueueFlagBits::TRANSFER_BIT,
            )
        })
        .nth(0)
        .unwrap()
        .0 as u32;

    println!("Using queue family {}", queue_family_index);

    let queue_priorities = &[1.0f32];
    let queue_create_infos = &[VkDeviceQueueCreateInfoBuilder::new()
        .queue_count(1)
        .queue_family_index(queue_family_index)
        .p_queue_priorities(queue_priorities)
        .build()];

    let create_info = VkDeviceCreateInfoBuilder::new().p_queue_create_infos(queue_create_infos);

    let vk_device = vk_instance
        .create_device(gpu, &create_info, None)
        .unwrap()
        .1;
    let vk_device = vk::Device::new(vk_device, &vk_instance);

    let vk_queue = vk_device.get_device_queue(queue_family_index, 0);

    'outer_loop: loop
    {
        while let Some(event) = window.poll_event()
        {
            match event
            {
                Event::DestroyWindow => break 'outer_loop,
            }
        }
    }

    vk_device.queue_wait_idle(vk_queue).unwrap();
    vk_device.device_wait_idle().unwrap();
    vk_device.destroy_device(None);
    vk_instance.destroy_surface_khr(vk_surface, None);
    vk_instance.destroy_instance(None);
}
