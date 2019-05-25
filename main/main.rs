mod wsi;

use core::mem::transmute;
use fnd::{
    alloc::{set_global_allocator, SystemAllocator},
    containers::Array,
    str::CStr,
    Unq,
};
use vk::{builders::*, types::*};
use win32::kernel32;
use wsi::{Event, Window};

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

fn find_best_image_count(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    vk_surface: VkSurfaceKHR,
    preferred_count: u32,
) -> Result<u32, VkResult>
{
    let surface_prps = vk_instance
        .get_physical_device_surface_capabilities_khr(gpu, vk_surface)?
        .1;

    return Ok(surface_prps
        .min_image_count
        .max(surface_prps.max_image_count.min(preferred_count)));
}

fn find_best_surface_format(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    vk_surface: VkSurfaceKHR,
    preferred_format: VkFormat,
    preferred_color_space: VkColorSpaceKHR,
) -> Result<(VkFormat, VkColorSpaceKHR), VkResult>
{
    let format_count = vk_instance
        .get_physical_device_surface_formats_khr_count(gpu, vk_surface)?
        .1;

    let mut formats = Array::new();
    formats.resize_default(format_count);

    vk_instance.get_physical_device_surface_formats_khr(gpu, vk_surface, &mut formats)?;

    return if format_count == 1 && formats[0].format == VkFormat::UNDEFINED
    {
        Ok((preferred_format, preferred_color_space))
    }
    else
    {
        Ok(formats
            .iter()
            .filter(|f| f.format == preferred_format && f.color_space == preferred_color_space)
            .nth(0)
            .map(|f| (f.format, f.color_space))
            .unwrap_or((
                VkFormat::B8G8R8A8_UNORM,
                VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
            )))
    };
}

fn find_best_present_mode(
    vk_instance: &vk::Instance,
    gpu: VkPhysicalDevice,
    vk_surface: VkSurfaceKHR,
    preferred_present_mode: VkPresentModeKHR,
) -> Result<VkPresentModeKHR, VkResult>
{
    let present_mode_count = vk_instance
        .get_physical_device_surface_present_modes_khr_count(gpu, vk_surface)?
        .1;
    let mut present_modes = Array::new();
    present_modes.resize_default(present_mode_count);

    vk_instance.get_physical_device_surface_present_modes_khr(
        gpu,
        vk_surface,
        &mut present_modes,
    )?;

    return Ok(present_modes
        .iter()
        .copied()
        .filter(|p| *p == preferred_present_mode)
        .nth(0)
        .unwrap_or(VkPresentModeKHR::FIFO_KHR));
}

extern "system" fn messenger_cb(
    _message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
    _message_types: VkDebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut core::ffi::c_void,
) -> VkBool32
{
    unsafe {
        let callback_data: &VkDebugUtilsMessengerCallbackDataEXT = &*p_callback_data;
        match CStr::from_bytes_null_terminated_unchecked(callback_data.p_message).as_str()
        {
            Ok(s) => println!("{}", s),
            _ =>
            {}
        }
    }

    return VK_FALSE;
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
        VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_SURFACE_EXTENSION_NAME__C.as_ptr(),
        VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C.as_ptr(),
    ];

    let layers = &[b"VK_LAYER_KHRONOS_validation\0".as_ptr()];

    let create_info = VkInstanceCreateInfoBuilder::new()
        .pp_enabled_extension_names(instance_extensions)
        .pp_enabled_layer_names(layers);

    let vk_instance = vk_entry.create_instance(&create_info, None).unwrap().1;
    let vk_instance = vk::Instance::new(vk_instance, &vk_entry);

    let create_info = VkDebugUtilsMessengerCreateInfoEXTBuilder::new()
        .message_severity(
            VkDebugUtilsMessageSeverityFlagBitsEXT::ERROR_BIT_EXT
                | VkDebugUtilsMessageSeverityFlagBitsEXT::WARNING_BIT_EXT,
        )
        .message_type(
            VkDebugUtilsMessageTypeFlagBitsEXT::GENERAL_BIT_EXT
                | VkDebugUtilsMessageTypeFlagBitsEXT::VALIDATION_BIT_EXT,
        )
        .pfn_user_callback(Some(messenger_cb));

    let debug_messenger = vk_instance
        .create_debug_utils_messenger_ext(&create_info, None)
        .unwrap()
        .1;

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
        .filter(|(index, prps)| {
            let queue_type_ok = prps.queue_flags.contains(
                VkQueueFlagBits::GRAPHICS_BIT
                    | VkQueueFlagBits::COMPUTE_BIT
                    | VkQueueFlagBits::TRANSFER_BIT,
            );
            let surface_ok = vk_instance
                .get_physical_device_surface_support_khr(gpu, *index as u32, vk_surface)
                .unwrap()
                .1;
            let present_ok = vk_instance
                .get_physical_device_win_32_presentation_support_khr(gpu, *index as u32)
                == VK_TRUE;

            return queue_type_ok && surface_ok && present_ok;
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

    let device_extensions = &[VK_KHR_SWAPCHAIN_EXTENSION_NAME__C.as_ptr()];

    let create_info = VkDeviceCreateInfoBuilder::new()
        .p_queue_create_infos(queue_create_infos)
        .pp_enabled_extension_names(device_extensions)
        .pp_enabled_layer_names(layers);

    let vk_device = vk_instance
        .create_device(gpu, &create_info, None)
        .unwrap()
        .1;
    let vk_device = vk::Device::new(vk_device, &vk_instance);
    println!("Device created");

    let queue_families = &[queue_family_index];

    let image_count = find_best_image_count(&vk_instance, gpu, vk_surface, 2).unwrap();
    let (format, color_space) = find_best_surface_format(
        &vk_instance,
        gpu,
        vk_surface,
        VkFormat::B8G8R8A8_UNORM,
        VkColorSpaceKHR::SRGB_NONLINEAR_KHR,
    )
    .unwrap();
    let present_mode =
        find_best_present_mode(&vk_instance, gpu, vk_surface, VkPresentModeKHR::MAILBOX_KHR)
            .unwrap();
    let extent = vk_instance
        .get_physical_device_surface_capabilities_khr(gpu, vk_surface)
        .unwrap()
        .1
        .current_extent;

    let create_info = VkSwapchainCreateInfoKHRBuilder::new()
        .surface(vk_surface)
        .min_image_count(image_count)
        .image_format(format)
        .image_color_space(color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(VkImageUsageFlagBits::COLOR_ATTACHMENT_BIT)
        .image_sharing_mode(VkSharingMode::EXCLUSIVE)
        .p_queue_family_indices(queue_families)
        .pre_transform(VkSurfaceTransformFlagBitsKHR::IDENTITY_BIT_KHR)
        .composite_alpha(VkCompositeAlphaFlagBitsKHR::OPAQUE_BIT_KHR)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(VkSwapchainKHR::null());

    let vk_swapchain = vk_device
        .create_swapchain_khr(&create_info, None)
        .unwrap()
        .1;
    println!("Swapchain created");

    let vk_queue = vk_device.get_device_queue(queue_family_index, 0);

    window.events_loop(|e| match *e
    {
        Event::DestroyWindow => false,
    });

    vk_device.queue_wait_idle(vk_queue).unwrap();
    vk_device.device_wait_idle().unwrap();
    vk_device.destroy_swapchain_khr(vk_swapchain, None);
    vk_device.destroy_device(None);
    vk_instance.destroy_surface_khr(vk_surface, None);
    vk_instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
    vk_instance.destroy_instance(None);
}
