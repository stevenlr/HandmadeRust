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

    pub fn build_device(self, queue_configs: &[QueueConfig])
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
