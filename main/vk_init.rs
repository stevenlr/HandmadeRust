use crate::wsi::Window;
use vk::{builders::*, types::*};

use fnd::{
    containers::{Array, HashMap},
    str::CStr,
};

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

pub trait InstanceBuilder
{
    type WithWsi: InstanceBuilder;
    type WithDebug: InstanceBuilder;
    type NextBuilder;

    fn enable_debug(self) -> Self::WithDebug;
    fn enable_wsi(self) -> Self::WithWsi;
    fn build(self) -> Self::NextBuilder;
}

pub struct BaseInstanceBuilder
{
    vk_entry: vk::EntryPoint,
    extensions: Array<*const u8>,
    layers: Array<*const u8>,
    enable_debug: bool,
}

pub fn init_vulkan(vk_entry: vk::EntryPoint) -> BaseInstanceBuilder
{
    BaseInstanceBuilder::new(vk_entry)
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

pub struct SurfaceInstanceBuilder
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

pub struct SurfaceBuilder
{
    gpu_builder: GpuBuilder,
}

impl SurfaceBuilder
{
    pub fn build_surface(mut self, window: &Window) -> GpuBuilder
    {
        let vk_surface = window
            .create_vk_surface(&self.gpu_builder.vk_instance)
            .unwrap();
        self.gpu_builder.surface = Some(vk_surface);
        return self.gpu_builder;
    }
}

pub struct GpuBuilder
{
    vk_entry: vk::EntryPoint,
    vk_instance: vk::Instance,
    debug_messenger: Option<VkDebugUtilsMessengerEXT>,
    surface: Option<VkSurfaceKHR>,
}

impl GpuBuilder
{
    pub fn choose_gpu(
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

pub struct DeviceBuilder
{
    vk_entry: vk::EntryPoint,
    vk_instance: vk::Instance,
    debug_messenger: Option<VkDebugUtilsMessengerEXT>,
    surface: Option<VkSurfaceKHR>,
    gpu: VkPhysicalDevice,
}

const MAX_QUEUES: usize = 4;

pub struct QueueConfig
{
    pub flags: VkQueueFlags,
    pub supports_present: bool,
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

    pub fn build_device(self, queue_configs: &[QueueConfig]) -> VulkanContext
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

        let queue_families: Array<_> = queue_families.iter().map(|q| q.unwrap()).collect();

        let mut queues_grouped = HashMap::<u32, usize>::new();
        for q in queue_families.iter()
        {
            let group = queues_grouped.find_mut(&q);
            match group
            {
                Some(g) => *g += 1,
                None =>
                {
                    queues_grouped.insert(*q, 1);
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

        let mut queues = Array::new();
        queues.reserve(queue_families.len());

        let mut queues_family_count = HashMap::<u32, usize>::new();

        for q in queue_families.iter()
        {
            let group = queues_family_count.find_mut(&q);
            let index = match group
            {
                Some(g) =>
                {
                    *g += 1;
                    *g - 1
                }
                None =>
                {
                    queues_family_count.insert(*q, 1);
                    0
                }
            };

            queues.push(vk_device.get_device_queue(*q, index as u32));
        }

        VulkanContext {
            entry: self.vk_entry,
            instance: self.vk_instance,
            debug_messenger: self.debug_messenger,
            surface: self.surface,
            gpu: self.gpu,
            device: vk_device,
            queue_families,
            queues,
            swapchain: None,
        }
    }
}

pub struct Swapchain
{
    swapchain: VkSwapchainKHR,
    format: VkFormat,
    images: Array<VkImage>,
    image_views: Array<VkImageView>,
}

pub struct VulkanContext
{
    entry: vk::EntryPoint,
    instance: vk::Instance,
    debug_messenger: Option<VkDebugUtilsMessengerEXT>,
    surface: Option<VkSurfaceKHR>,
    gpu: VkPhysicalDevice,
    device: vk::Device,
    queue_families: Array<u32>,
    queues: Array<VkQueue>,
    swapchain: Option<Swapchain>,
}

pub struct SwapchainParams
{
    pub queue_index: usize,
    pub image_count: usize,
    pub format: VkFormat,
    pub color_space: VkColorSpaceKHR,
    pub present_mode: VkPresentModeKHR,
}

impl VulkanContext
{
    fn find_best_image_count(&self, preferred_count: u32) -> Result<u32, VkResult>
    {
        let surface_prps = self
            .instance
            .get_physical_device_surface_capabilities_khr(self.gpu, self.surface.unwrap())?
            .1;

        return Ok(surface_prps
            .min_image_count
            .max(surface_prps.max_image_count.min(preferred_count)));
    }

    fn find_best_surface_format(
        &self,
        preferred_format: VkFormat,
        preferred_color_space: VkColorSpaceKHR,
    ) -> Result<(VkFormat, VkColorSpaceKHR), VkResult>
    {
        let format_count = self
            .instance
            .get_physical_device_surface_formats_khr_count(self.gpu, self.surface.unwrap())?
            .1;

        let mut formats = Array::new();
        formats.resize_default(format_count);

        self.instance.get_physical_device_surface_formats_khr(
            self.gpu,
            self.surface.unwrap(),
            &mut formats,
        )?;

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
        &self,
        preferred_present_mode: VkPresentModeKHR,
    ) -> Result<VkPresentModeKHR, VkResult>
    {
        let present_mode_count = self
            .instance
            .get_physical_device_surface_present_modes_khr_count(self.gpu, self.surface.unwrap())?
            .1;

        let mut present_modes = Array::new();
        present_modes.resize_default(present_mode_count);

        self.instance
            .get_physical_device_surface_present_modes_khr(
                self.gpu,
                self.surface.unwrap(),
                &mut present_modes,
            )?;

        return Ok(present_modes
            .iter()
            .copied()
            .filter(|p| *p == preferred_present_mode)
            .nth(0)
            .unwrap_or(VkPresentModeKHR::FIFO_KHR));
    }

    pub fn init_swapchain(&mut self, params: &SwapchainParams)
    {
        assert!(self.surface.is_some());

        let queue_families = &[self.queue_families[params.queue_index]];

        let image_count = self
            .find_best_image_count(params.image_count as u32)
            .unwrap();

        let (format, color_space) = self
            .find_best_surface_format(params.format, params.color_space)
            .unwrap();

        let present_mode = self.find_best_present_mode(params.present_mode).unwrap();

        let extent = self
            .instance
            .get_physical_device_surface_capabilities_khr(self.gpu, self.surface.unwrap())
            .unwrap()
            .1
            .current_extent;

        let create_info = VkSwapchainCreateInfoKHRBuilder::new()
            .surface(self.surface.unwrap())
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

        let swapchain = self
            .device
            .create_swapchain_khr(&create_info, None)
            .unwrap()
            .1;

        let image_count = self
            .device
            .get_swapchain_images_khr_count(swapchain)
            .unwrap()
            .1;

        let mut images = Array::new();
        images.resize_default(image_count);

        self.device
            .get_swapchain_images_khr(swapchain, &mut images)
            .unwrap();

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
                return self.device.create_image_view(&create_info, None).unwrap().1;
            })
            .collect();

        let swapchain = Swapchain {
            swapchain,
            images,
            image_views,
            format,
        };

        self.swapchain = Some(swapchain);
    }
}

impl Drop for VulkanContext
{
    fn drop(&mut self)
    {
        for queue in self.queues.iter()
        {
            self.device.queue_wait_idle(*queue).unwrap();
        }

        self.device.device_wait_idle().unwrap();

        if let Some(swapchain) = &self.swapchain
        {
            swapchain.image_views.iter().for_each(|view| {
                self.device.destroy_image_view(*view, None);
            });

            self.device.destroy_swapchain_khr(swapchain.swapchain, None);
        }

        self.device.destroy_device(None);

        if let Some(surface) = self.surface
        {
            self.instance.destroy_surface_khr(surface, None);
        }

        if let Some(debug_messenger) = self.debug_messenger
        {
            self.instance
                .destroy_debug_utils_messenger_ext(debug_messenger, None);
        }

        self.instance.destroy_instance(None);
    }
}
