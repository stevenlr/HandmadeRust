extern crate fnd;
extern crate vk;
extern crate win32;

use core::mem::transmute;

use fnd::alloc::Win32HeapAllocator;
use fnd::alloc::Allocator;
use fnd::containers::Array;
use fnd::str::CStr;

use vk::builders::*;
use vk::types::*;

use win32::kernel32::*;

fn get_gpu_queue_family_properties<A: Allocator>(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    alloc: A) -> Array<VkQueueFamilyProperties, A>
{
    let mut prps = Array::new(alloc);

    prps.resize(
        vk_instance.get_physical_device_queue_family_properties_count(gpu),
        VkQueueFamilyProperties::default());
    vk_instance.get_physical_device_queue_family_properties(gpu, &mut prps);

    return prps;
}


fn main()
{
    let allocator = Win32HeapAllocator::default();

    let vk_module = unsafe { LoadLibraryA(b"vulkan-1.dll\0".as_ptr()) };
    let vk_entry = vk::EntryPoint::new(|fn_name| unsafe
    {
        transmute(GetProcAddress(vk_module, fn_name.as_ptr()))
    });

    let instance_extensions = &[
        //VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_SURFACE_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C.as_ptr(),
    ];

    let layers = &[
        b"VK_LAYER_LUNARG_standard_validation\0".as_ptr(),
    ];

    let create_info = VkInstanceCreateInfoBuilder::new()
        .pp_enabled_extension_names(instance_extensions)
        .pp_enabled_layer_names(layers);

    let vk_instance = vk_entry.create_instance(&create_info, None).unwrap().1;
    let vk_instance = vk::Instance::new(vk_instance, &vk_entry);

    let gpu_count = vk_instance.enumerate_physical_devices_count().unwrap().1;
    println!("{} GPU(s)", gpu_count);

    let gpus =
    {
        let mut gpus = Array::new(&allocator);
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
        // .hinstance( ... )
        /*.hwnd( ... )*/;

    let vk_surface = vk_instance.create_win_32_surface_khr(&create_info, None).unwrap().1;

    let gpu = *gpus.iter()
        .nth(0)
        .unwrap();
    println!("Using GPU 0");

    let queue_family_properties = get_gpu_queue_family_properties(&vk_instance, gpu, &allocator);
    let queue_family_index = queue_family_properties.iter()
        .enumerate()
        .filter(|(_, prps)|
        {
            prps.queue_flags.contains(
                VkQueueFlagBits::GRAPHICS_BIT |
                VkQueueFlagBits::COMPUTE_BIT |
                VkQueueFlagBits::TRANSFER_BIT)
        })
        .nth(0)
        .unwrap().0 as u32;

    println!("Using queue family {}", queue_family_index);

    let queue_priorities = &[1.0f32];
    let queue_create_infos = &[
        VkDeviceQueueCreateInfoBuilder::new()
            .queue_count(1)
            .queue_family_index(queue_family_index)
            .p_queue_priorities(queue_priorities)
            .build(),
    ];

    let create_info = VkDeviceCreateInfoBuilder::new()
        .p_queue_create_infos(queue_create_infos);

    let vk_device = vk_instance.create_device(gpu, &create_info, None).unwrap().1;
    let vk_device = vk::Device::new(vk_device, &vk_instance);

    let vk_queue = vk_device.get_device_queue(queue_family_index, 0);

    vk_device.queue_wait_idle(vk_queue).unwrap();
    vk_device.device_wait_idle().unwrap();
    vk_device.destroy_device(None);
    vk_instance.destroy_surface_khr(vk_surface, None);
    vk_instance.destroy_instance(None);
}
