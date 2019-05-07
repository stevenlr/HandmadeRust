use core::mem::transmute;
use core::ptr::{null, null_mut};

use fnd::alloc::{set_global_allocator, SystemAllocator};
use fnd::containers::Array;
use fnd::str::CStr;
use fnd::Unq;

use vk::builders::*;
use vk::types::*;

use win32::kernel32;
use win32::user32;

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

fn main()
{
    init_global_allocator();

    let hinstance = unsafe { kernel32::GetModuleHandleA(0 as _) as win32::HINSTANCE };

    let wnd_class_name = b"HandmadeRustClass\0".as_ptr();

    let wnd_class = win32::WNDCLASSA {
        style: win32::CS_VREDRAW | win32::CS_HREDRAW,
        lpfnWndProc: user32::DefWindowProcA,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null(),
        lpszClassName: wnd_class_name,
    };

    let window = unsafe {
        user32::RegisterClassA(&wnd_class);
        user32::CreateWindowExA(
            0,
            wnd_class_name,
            b"Handmade Rust\0".as_ptr(),
            win32::WS_OVERLAPPEDWINDOW,
            win32::CW_USEDEFAULT,
            win32::CW_USEDEFAULT,
            1280,
            720,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        )
    };

    assert!(window != null_mut());
    unsafe {
        user32::ShowWindow(window, win32::SW_SHOW);
    }

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

    let create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
        .hinstance(hinstance)
        .hwnd(window);

    let vk_surface = vk_instance
        .create_win_32_surface_khr(&create_info, None)
        .unwrap()
        .1;

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

    loop
    {
        let mut msg: win32::MSG = unsafe { core::mem::zeroed() };
        while unsafe { user32::PeekMessageA(&mut msg, window, 0, 0, win32::PM_REMOVE) } > 0
        {
            unsafe {
                user32::TranslateMessage(&msg);
                user32::DispatchMessageA(&msg);
            }
        }
    }

    vk_device.queue_wait_idle(vk_queue).unwrap();
    vk_device.device_wait_idle().unwrap();
    vk_device.destroy_device(None);
    vk_instance.destroy_surface_khr(vk_surface, None);
    vk_instance.destroy_instance(None);
}
