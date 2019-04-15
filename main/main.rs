extern crate fnd;
extern crate vk;

use core::ffi::c_void;
use core::mem::transmute;
use core::slice::from_raw_parts;
use core::str::from_utf8_unchecked;

use fnd::alloc::Win32HeapAllocator;
use fnd::containers::Array;

use vk::builders::*;
use vk::types::*;

type HModule = usize;

#[link(name = "kernel32")]
extern "system"
{
    fn LoadLibraryA(lib_name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, fn_name: *const u8) -> *const c_void;
}

unsafe fn byte_slice_from_null_terminated<'a>(start: *const u8) -> &'a [u8]
{
    let mut ptr = start;
    let mut size : usize = 0;

    while ptr.read() != 0
    {
        size += 1;
        ptr = ptr.offset(1);
    }

    return from_raw_parts(start, size);
}

unsafe fn str_from_null_terminated_unchecked<'a>(start: *const u8) -> &'a str
{
    let slice = byte_slice_from_null_terminated(start);
    return from_utf8_unchecked(slice);
}

fn main()
{
    let allocator = Win32HeapAllocator::default();

    let vk_module = unsafe { LoadLibraryA(b"vulkan-1.dll\0".as_ptr()) };
    let vk_entry = vk::EntryPoint::new(|fn_name| unsafe
    {
        transmute(GetProcAddress(vk_module, fn_name.as_ptr()))
    });

    let create_info = VkInstanceCreateInfoBuilder::new();
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
        let name_str = unsafe { str_from_null_terminated_unchecked(prps.device_name.as_ptr()) };
        println!("    {}: {}", index, name_str);
    }

    vk_instance.destroy_instance(None);
}
