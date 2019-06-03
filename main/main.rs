#![feature(proc_macro_hygiene)]

mod wsi;

use core::mem::transmute;
use fnd::{
    alloc::{set_global_allocator, SystemAllocator},
    containers::{Array, HashMap},
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

trait InstanceBuilder
{
    type WithWsi: InstanceBuilder;
    type WithDebug: InstanceBuilder;
    type NextBuilder;

    fn enable_debug(self) -> Self::WithDebug;
    fn enable_wsi(self) -> Self::WithWsi;
    fn build(self) -> Self::NextBuilder;
}

struct BaseInstanceBuilder
{
    vk_entry: vk::EntryPoint,
    extensions: Array<*const u8>,
    layers: Array<*const u8>,
    enable_debug: bool,
}

impl BaseInstanceBuilder
{
    pub fn new(vk_entry: vk::EntryPoint) -> Self
    {
        Self {
            vk_entry,
            extensions: Array::new(),
            layers: Array::new(),
            enable_debug: false,
        }
    }

    fn build_instance(&self) -> vk::Instance
    {
        let create_info = VkInstanceCreateInfoBuilder::new()
            .pp_enabled_extension_names(&self.extensions)
            .pp_enabled_layer_names(&self.layers);

        let vk_instance = self.vk_entry.create_instance(&create_info, None).unwrap().1;
        return vk::Instance::new(vk_instance, &self.vk_entry);
    }

    fn build_debug(&self, vk_instance: &vk::Instance) -> Option<VkDebugUtilsMessengerEXT>
    {
        if self.enable_debug
        {
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

            return Some(
                vk_instance
                    .create_debug_utils_messenger_ext(&create_info, None)
                    .unwrap()
                    .1,
            );
        }
        else
        {
            None
        }
    }
}

impl InstanceBuilder for BaseInstanceBuilder
{
    type WithDebug = Self;
    type WithWsi = SurfaceInstanceBuilder;
    type NextBuilder = GpuBuilder;

    fn enable_debug(mut self) -> Self
    {
        self.extensions
            .push(VK_EXT_DEBUG_UTILS_EXTENSION_NAME__C.as_ptr());
        self.layers.push(b"VK_LAYER_KHRONOS_validation\0".as_ptr());
        self.enable_debug = true;
        return self;
    }

    fn enable_wsi(mut self) -> Self::WithWsi
    {
        self.extensions
            .push(VK_KHR_SURFACE_EXTENSION_NAME__C.as_ptr());
        self.extensions
            .push(VK_KHR_WIN32_SURFACE_EXTENSION_NAME__C.as_ptr());

        return Self::WithWsi { base: self };
    }

    fn build(self) -> Self::NextBuilder
    {
        let vk_instance = self.build_instance();
        let debug_messenger = self.build_debug(&vk_instance);

        return GpuBuilder {
            vk_entry: self.vk_entry,
            vk_instance,
            debug_messenger,
            surface: None,
        };
    }
}

struct SurfaceInstanceBuilder
{
    base: BaseInstanceBuilder,
}

impl InstanceBuilder for SurfaceInstanceBuilder
{
    type WithDebug = Self;
    type WithWsi = Self;
    type NextBuilder = SurfaceBuilder;

    fn enable_debug(mut self) -> Self
    {
        self.base = self.base.enable_debug();
        return self;
    }

    fn enable_wsi(self) -> Self::WithWsi
    {
        self
    }

    fn build(self) -> Self::NextBuilder
    {
        SurfaceBuilder {
            gpu_builder: self.base.build(),
        }
    }
}

struct SurfaceBuilder
{
    gpu_builder: GpuBuilder,
}

impl SurfaceBuilder
{
    fn build_surface(mut self, window: &Window) -> GpuBuilder
    {
        let vk_surface = window
            .create_vk_surface(&self.gpu_builder.vk_instance)
            .unwrap();
        self.gpu_builder.surface = Some(vk_surface);
        return self.gpu_builder;
    }
}

struct GpuBuilder
{
    vk_entry: vk::EntryPoint,
    vk_instance: vk::Instance,
    debug_messenger: Option<VkDebugUtilsMessengerEXT>,
    surface: Option<VkSurfaceKHR>,
}

impl GpuBuilder
{
    fn choose_gpu(
        self,
        choose_fn: impl Fn(VkPhysicalDevice, &VkPhysicalDeviceProperties) -> bool,
    ) -> DeviceBuilder
    {
        let gpu_count = self
            .vk_instance
            .enumerate_physical_devices_count()
            .unwrap()
            .1;

        let mut gpus = Array::new();
        gpus.resize_default(gpu_count);

        self.vk_instance
            .enumerate_physical_devices(&mut gpus)
            .unwrap();

        let gpu = gpus
            .iter()
            .map(|gpu| {
                let prps = self.vk_instance.get_physical_device_properties(*gpu);

                let queue_count = self
                    .vk_instance
                    .get_physical_device_queue_family_properties_count(*gpu);

                let mut queues = Array::new();
                queues.resize_default(queue_count);

                self.vk_instance
                    .get_physical_device_queue_family_properties(*gpu, &mut queues);

                (*gpu, prps, queues)
            })
            .filter(|(gpu, prps, queues)| {
                if let Some(surface) = self.surface
                {
                    let supports_surface = queues.iter().enumerate().any(|(index, _)| {
                        self.vk_instance
                            .get_physical_device_surface_support_khr(*gpu, index as u32, surface)
                            .unwrap()
                            .1
                    });

                    if !supports_surface
                    {
                        return false;
                    }
                }

                choose_fn(*gpu, prps)
            })
            .nth(0)
            .unwrap()
            .0;

        DeviceBuilder {
            vk_entry: self.vk_entry,
            vk_instance: self.vk_instance,
            debug_messenger: self.debug_messenger,
            surface: self.surface,
            gpu,
        }
    }
}

struct DeviceBuilder
{
    vk_entry: vk::EntryPoint,
    vk_instance: vk::Instance,
    debug_messenger: Option<VkDebugUtilsMessengerEXT>,
    surface: Option<VkSurfaceKHR>,
    gpu: VkPhysicalDevice,
}

const MAX_QUEUES: usize = 4;

struct QueueConfig
{
    flags: VkQueueFlags,
    supports_present: bool,
}

impl DeviceBuilder
{
    fn find_queue_index(
        &self,
        criteria: &QueueConfig,
        queues: &mut [VkQueueFamilyProperties],
    ) -> Option<u32>
    {
        let index = queues
            .iter()
            .enumerate()
            .filter(|(_, prps)| prps.queue_flags.contains(criteria.flags))
            .filter(|(index, _)| {
                if criteria.supports_present
                {
                    if let Some(surface) = self.surface
                    {
                        return self
                            .vk_instance
                            .get_physical_device_surface_support_khr(
                                self.gpu,
                                *index as u32,
                                surface,
                            )
                            .unwrap()
                            .1;
                    }
                    else
                    {
                        return false;
                    }
                }
                else
                {
                    return true;
                }
            })
            .map(|(index, _)| index as u32)
            .nth(0);

        if let Some(index) = index
        {
            queues[index as usize].queue_count -= 1;
        }

        return index;
    }

    fn build_device(self, queue_configs: &[QueueConfig])
    {
        assert!(queue_configs.len() <= MAX_QUEUES);

        let queue_count = self
            .vk_instance
            .get_physical_device_queue_family_properties_count(self.gpu);

        let mut queues = Array::new();
        queues.resize_default(queue_count);

        self.vk_instance
            .get_physical_device_queue_family_properties(self.gpu, &mut queues);

        let queue_families: Array<_> = queue_configs
            .iter()
            .map(|config| self.find_queue_index(config, &mut queues))
            .collect();

        if queue_families.iter().any(|q| q.is_none())
        {
            panic!("Not all queue families were found");
        }

        let mut queues_grouped = HashMap::<u32, usize>::new();
        for q in queue_families
        {
            let group = queues_grouped.find_mut(&q.unwrap());
            match group
            {
                Some(g) => *g += 1,
                None =>
                {
                    queues_grouped.insert(q.unwrap(), 1);
                }
            }
        }

        let priorities = [1.0f32, 1.0f32, 1.0f32, 1.0f32];

        let queues_create_info: Array<_> = queues_grouped
            .keys_values()
            .map(|(index, count)| {
                VkDeviceQueueCreateInfoBuilder::new()
                    .p_queue_priorities(&priorities[0..*count])
                    .queue_count(*count as u32)
                    .queue_family_index(*index as u32)
                    .build()
            })
            .collect();

        let mut device_extensions = Array::new();
        if self.surface.is_some()
        {
            device_extensions.push(VK_KHR_SWAPCHAIN_EXTENSION_NAME__C.as_ptr());
        }

        let create_info = VkDeviceCreateInfoBuilder::new()
            .p_queue_create_infos(&queues_create_info)
            .pp_enabled_extension_names(&device_extensions);

        let vk_device = self
            .vk_instance
            .create_device(self.gpu, &create_info, None)
            .unwrap()
            .1;

        let vk_device = vk::Device::new(vk_device, &self.vk_instance);

        // @Todo Return all the necessary informations
    }
}

fn main()
{
    let a = hash_macro::murmur3_32!("Hello world!");
    println!("{}", a);

    init_global_allocator();

    let window = Window::new(1280, 720, "Handmade Rust").unwrap();

    let vk_module = unsafe { kernel32::LoadLibraryA(b"vulkan-1.dll\0".as_ptr()) };
    let vk_entry = vk::EntryPoint::new(|fn_name| unsafe {
        transmute(kernel32::GetProcAddress(vk_module, fn_name.as_ptr()))
    });

    let queue_configs = [QueueConfig {
        flags: VkQueueFlags::GRAPHICS_BIT | VkQueueFlags::TRANSFER_BIT | VkQueueFlags::COMPUTE_BIT,
        supports_present: true,
    }];

    BaseInstanceBuilder::new(vk_entry)
        .enable_debug()
        .enable_wsi()
        .build()
        .build_surface(&window)
        .choose_gpu(|_, prps: &VkPhysicalDeviceProperties| {
            prps.device_type == VkPhysicalDeviceType::DISCRETE_GPU
        })
        .build_device(&queue_configs);

    /*



    let queue_families = &[queue_family_index];

    let image_count = find_best_image_count(&vk_instance, gpu, vk_surface, 3).unwrap();
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

    let image_count = vk_device
        .get_swapchain_images_khr_count(vk_swapchain)
        .unwrap()
        .1;
    let mut images = Array::new();
    images.resize_default(image_count);

    vk_device
        .get_swapchain_images_khr(vk_swapchain, &mut images)
        .unwrap();
    println!("    Swapchain uses {} images", images.len());

    let create_info = VkImageViewCreateInfoBuilder::new()
        .view_type(VkImageViewType::K_2D)
        .format(format)
        .components(
            VkComponentMappingBuilder::new()
                .r(VkComponentSwizzle::R)
                .g(VkComponentSwizzle::G)
                .b(VkComponentSwizzle::B)
                .a(VkComponentSwizzle::A)
                .build(),
        )
        .subresource_range(
            VkImageSubresourceRangeBuilder::new()
                .aspect_mask(VkImageAspectFlags::COLOR_BIT)
                .base_mip_level(0)
                .base_array_layer(0)
                .level_count(1)
                .layer_count(1)
                .build(),
        );

    let image_views: Array<VkImageView> = images
        .iter()
        .map(|img| {
            let create_info = create_info.clone().image(*img);
            return vk_device.create_image_view(&create_info, None).unwrap().1;
        })
        .collect();

    let vk_queue = vk_device.get_device_queue(queue_family_index, 0);

    window.events_loop(|e| match *e
    {
        Event::DestroyWindow => false,
    });

    vk_device.queue_wait_idle(vk_queue).unwrap();
    vk_device.device_wait_idle().unwrap();

    image_views.into_iter().for_each(|view| {
        vk_device.destroy_image_view(view, None);
    });

    vk_device.destroy_swapchain_khr(vk_swapchain, None);
    vk_device.destroy_device(None);
    vk_instance.destroy_surface_khr(vk_surface, None);
    vk_instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
    vk_instance.destroy_instance(None);
    */
}
