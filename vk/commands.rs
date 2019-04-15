use crate::types::*;

#[derive(Clone)]
pub struct StaticCommands {
    pfn_get_instance_proc_addr: PfnVkGetInstanceProcAddr,
}

impl StaticCommands {
    pub fn load(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {
        StaticCommands {
            pfn_get_instance_proc_addr: unsafe { core::mem::transmute(load_fn(b"vkGetInstanceProcAddr\0")) },
        }
    }

    #[inline]
    pub unsafe fn get_instance_proc_addr(&self,
        instance: VkInstance,
        p_name: *const u8) -> PfnVkVoidFunction {
        (self.pfn_get_instance_proc_addr)(
            instance,
            p_name,)
    }
}

#[derive(Clone)]
pub struct EntryCommands {
    pfn_enumerate_instance_layer_properties: PfnVkEnumerateInstanceLayerProperties,
    pfn_enumerate_instance_extension_properties: PfnVkEnumerateInstanceExtensionProperties,
    pfn_create_instance: PfnVkCreateInstance,
}

impl EntryCommands {
    pub fn load(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {
        EntryCommands {
            pfn_enumerate_instance_layer_properties: unsafe { core::mem::transmute(load_fn(b"vkEnumerateInstanceLayerProperties\0")) },
            pfn_enumerate_instance_extension_properties: unsafe { core::mem::transmute(load_fn(b"vkEnumerateInstanceExtensionProperties\0")) },
            pfn_create_instance: unsafe { core::mem::transmute(load_fn(b"vkCreateInstance\0")) },
        }
    }

    #[inline]
    pub unsafe fn enumerate_instance_layer_properties(&self,
        p_property_count: *mut u32,
        p_properties: *mut VkLayerProperties) -> VkResult {
        (self.pfn_enumerate_instance_layer_properties)(
            p_property_count,
            p_properties,)
    }

    #[inline]
    pub unsafe fn enumerate_instance_extension_properties(&self,
        p_layer_name: *const u8,
        p_property_count: *mut u32,
        p_properties: *mut VkExtensionProperties) -> VkResult {
        (self.pfn_enumerate_instance_extension_properties)(
            p_layer_name,
            p_property_count,
            p_properties,)
    }

    #[inline]
    pub unsafe fn create_instance(&self,
        p_create_info: *const VkInstanceCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_instance: *mut VkInstance) -> VkResult {
        (self.pfn_create_instance)(
            p_create_info,
            p_allocator,
            p_instance,)
    }
}

#[derive(Clone)]
pub struct InstanceCommands {
    pfn_get_physical_device_win_32_presentation_support_khr: PfnVkGetPhysicalDeviceWin32PresentationSupportKHR,
    pfn_create_win_32_surface_khr: PfnVkCreateWin32SurfaceKHR,
    pfn_get_physical_device_surface_present_modes_khr: PfnVkGetPhysicalDeviceSurfacePresentModesKHR,
    pfn_get_physical_device_surface_formats_khr: PfnVkGetPhysicalDeviceSurfaceFormatsKHR,
    pfn_get_physical_device_surface_capabilities_khr: PfnVkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pfn_get_physical_device_surface_support_khr: PfnVkGetPhysicalDeviceSurfaceSupportKHR,
    pfn_destroy_surface_khr: PfnVkDestroySurfaceKHR,
    pfn_submit_debug_utils_message_ext: PfnVkSubmitDebugUtilsMessageEXT,
    pfn_destroy_debug_utils_messenger_ext: PfnVkDestroyDebugUtilsMessengerEXT,
    pfn_create_debug_utils_messenger_ext: PfnVkCreateDebugUtilsMessengerEXT,
    pfn_cmd_insert_debug_utils_label_ext: PfnVkCmdInsertDebugUtilsLabelEXT,
    pfn_cmd_end_debug_utils_label_ext: PfnVkCmdEndDebugUtilsLabelEXT,
    pfn_cmd_begin_debug_utils_label_ext: PfnVkCmdBeginDebugUtilsLabelEXT,
    pfn_queue_insert_debug_utils_label_ext: PfnVkQueueInsertDebugUtilsLabelEXT,
    pfn_queue_end_debug_utils_label_ext: PfnVkQueueEndDebugUtilsLabelEXT,
    pfn_queue_begin_debug_utils_label_ext: PfnVkQueueBeginDebugUtilsLabelEXT,
    pfn_set_debug_utils_object_tag_ext: PfnVkSetDebugUtilsObjectTagEXT,
    pfn_set_debug_utils_object_name_ext: PfnVkSetDebugUtilsObjectNameEXT,
    pfn_get_physical_device_sparse_image_format_properties: PfnVkGetPhysicalDeviceSparseImageFormatProperties,
    pfn_enumerate_device_layer_properties: PfnVkEnumerateDeviceLayerProperties,
    pfn_enumerate_device_extension_properties: PfnVkEnumerateDeviceExtensionProperties,
    pfn_create_device: PfnVkCreateDevice,
    pfn_get_device_proc_addr: PfnVkGetDeviceProcAddr,
    pfn_get_physical_device_memory_properties: PfnVkGetPhysicalDeviceMemoryProperties,
    pfn_get_physical_device_queue_family_properties: PfnVkGetPhysicalDeviceQueueFamilyProperties,
    pfn_get_physical_device_properties: PfnVkGetPhysicalDeviceProperties,
    pfn_get_physical_device_image_format_properties: PfnVkGetPhysicalDeviceImageFormatProperties,
    pfn_get_physical_device_format_properties: PfnVkGetPhysicalDeviceFormatProperties,
    pfn_get_physical_device_features: PfnVkGetPhysicalDeviceFeatures,
    pfn_enumerate_physical_devices: PfnVkEnumeratePhysicalDevices,
    pfn_destroy_instance: PfnVkDestroyInstance,
}

impl InstanceCommands {
    pub fn load(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {
        InstanceCommands {
            pfn_get_physical_device_win_32_presentation_support_khr: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0")) },
            pfn_create_win_32_surface_khr: unsafe { core::mem::transmute(load_fn(b"vkCreateWin32SurfaceKHR\0")) },
            pfn_get_physical_device_surface_present_modes_khr: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceSurfacePresentModesKHR\0")) },
            pfn_get_physical_device_surface_formats_khr: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceSurfaceFormatsKHR\0")) },
            pfn_get_physical_device_surface_capabilities_khr: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0")) },
            pfn_get_physical_device_surface_support_khr: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceSurfaceSupportKHR\0")) },
            pfn_destroy_surface_khr: unsafe { core::mem::transmute(load_fn(b"vkDestroySurfaceKHR\0")) },
            pfn_submit_debug_utils_message_ext: unsafe { core::mem::transmute(load_fn(b"vkSubmitDebugUtilsMessageEXT\0")) },
            pfn_destroy_debug_utils_messenger_ext: unsafe { core::mem::transmute(load_fn(b"vkDestroyDebugUtilsMessengerEXT\0")) },
            pfn_create_debug_utils_messenger_ext: unsafe { core::mem::transmute(load_fn(b"vkCreateDebugUtilsMessengerEXT\0")) },
            pfn_cmd_insert_debug_utils_label_ext: unsafe { core::mem::transmute(load_fn(b"vkCmdInsertDebugUtilsLabelEXT\0")) },
            pfn_cmd_end_debug_utils_label_ext: unsafe { core::mem::transmute(load_fn(b"vkCmdEndDebugUtilsLabelEXT\0")) },
            pfn_cmd_begin_debug_utils_label_ext: unsafe { core::mem::transmute(load_fn(b"vkCmdBeginDebugUtilsLabelEXT\0")) },
            pfn_queue_insert_debug_utils_label_ext: unsafe { core::mem::transmute(load_fn(b"vkQueueInsertDebugUtilsLabelEXT\0")) },
            pfn_queue_end_debug_utils_label_ext: unsafe { core::mem::transmute(load_fn(b"vkQueueEndDebugUtilsLabelEXT\0")) },
            pfn_queue_begin_debug_utils_label_ext: unsafe { core::mem::transmute(load_fn(b"vkQueueBeginDebugUtilsLabelEXT\0")) },
            pfn_set_debug_utils_object_tag_ext: unsafe { core::mem::transmute(load_fn(b"vkSetDebugUtilsObjectTagEXT\0")) },
            pfn_set_debug_utils_object_name_ext: unsafe { core::mem::transmute(load_fn(b"vkSetDebugUtilsObjectNameEXT\0")) },
            pfn_get_physical_device_sparse_image_format_properties: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceSparseImageFormatProperties\0")) },
            pfn_enumerate_device_layer_properties: unsafe { core::mem::transmute(load_fn(b"vkEnumerateDeviceLayerProperties\0")) },
            pfn_enumerate_device_extension_properties: unsafe { core::mem::transmute(load_fn(b"vkEnumerateDeviceExtensionProperties\0")) },
            pfn_create_device: unsafe { core::mem::transmute(load_fn(b"vkCreateDevice\0")) },
            pfn_get_device_proc_addr: unsafe { core::mem::transmute(load_fn(b"vkGetDeviceProcAddr\0")) },
            pfn_get_physical_device_memory_properties: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceMemoryProperties\0")) },
            pfn_get_physical_device_queue_family_properties: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceQueueFamilyProperties\0")) },
            pfn_get_physical_device_properties: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceProperties\0")) },
            pfn_get_physical_device_image_format_properties: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceImageFormatProperties\0")) },
            pfn_get_physical_device_format_properties: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceFormatProperties\0")) },
            pfn_get_physical_device_features: unsafe { core::mem::transmute(load_fn(b"vkGetPhysicalDeviceFeatures\0")) },
            pfn_enumerate_physical_devices: unsafe { core::mem::transmute(load_fn(b"vkEnumeratePhysicalDevices\0")) },
            pfn_destroy_instance: unsafe { core::mem::transmute(load_fn(b"vkDestroyInstance\0")) },
        }
    }

    #[inline]
    pub unsafe fn get_physical_device_win_32_presentation_support_khr(&self,
        physical_device: VkPhysicalDevice,
        queue_family_index: u32) -> VkBool32 {
        (self.pfn_get_physical_device_win_32_presentation_support_khr)(
            physical_device,
            queue_family_index,)
    }

    #[inline]
    pub unsafe fn create_win_32_surface_khr(&self,
        instance: VkInstance,
        p_create_info: *const VkWin32SurfaceCreateInfoKHR,
        p_allocator: *const VkAllocationCallbacks,
        p_surface: *mut VkSurfaceKHR) -> VkResult {
        (self.pfn_create_win_32_surface_khr)(
            instance,
            p_create_info,
            p_allocator,
            p_surface,)
    }

    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes_khr(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut VkPresentModeKHR) -> VkResult {
        (self.pfn_get_physical_device_surface_present_modes_khr)(
            physical_device,
            surface,
            p_present_mode_count,
            p_present_modes,)
    }

    #[inline]
    pub unsafe fn get_physical_device_surface_formats_khr(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut VkSurfaceFormatKHR) -> VkResult {
        (self.pfn_get_physical_device_surface_formats_khr)(
            physical_device,
            surface,
            p_surface_format_count,
            p_surface_formats,)
    }

    #[inline]
    pub unsafe fn get_physical_device_surface_capabilities_khr(&self,
        physical_device: VkPhysicalDevice,
        surface: VkSurfaceKHR,
        p_surface_capabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult {
        (self.pfn_get_physical_device_surface_capabilities_khr)(
            physical_device,
            surface,
            p_surface_capabilities,)
    }

    #[inline]
    pub unsafe fn get_physical_device_surface_support_khr(&self,
        physical_device: VkPhysicalDevice,
        queue_family_index: u32,
        surface: VkSurfaceKHR,
        p_supported: *mut VkBool32) -> VkResult {
        (self.pfn_get_physical_device_surface_support_khr)(
            physical_device,
            queue_family_index,
            surface,
            p_supported,)
    }

    #[inline]
    pub unsafe fn destroy_surface_khr(&self,
        instance: VkInstance,
        surface: VkSurfaceKHR,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_surface_khr)(
            instance,
            surface,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn submit_debug_utils_message_ext(&self,
        instance: VkInstance,
        message_severity: VkDebugUtilsMessageSeverityFlagBitsEXT,
        message_types: VkDebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const VkDebugUtilsMessengerCallbackDataEXT) {
        (self.pfn_submit_debug_utils_message_ext)(
            instance,
            message_severity,
            message_types,
            p_callback_data,)
    }

    #[inline]
    pub unsafe fn destroy_debug_utils_messenger_ext(&self,
        instance: VkInstance,
        messenger: VkDebugUtilsMessengerEXT,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_debug_utils_messenger_ext)(
            instance,
            messenger,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_debug_utils_messenger_ext(&self,
        instance: VkInstance,
        p_create_info: *const VkDebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const VkAllocationCallbacks,
        p_messenger: *mut VkDebugUtilsMessengerEXT) -> VkResult {
        (self.pfn_create_debug_utils_messenger_ext)(
            instance,
            p_create_info,
            p_allocator,
            p_messenger,)
    }

    #[inline]
    pub unsafe fn cmd_insert_debug_utils_label_ext(&self,
        command_buffer: VkCommandBuffer,
        p_label_info: *const VkDebugUtilsLabelEXT) {
        (self.pfn_cmd_insert_debug_utils_label_ext)(
            command_buffer,
            p_label_info,)
    }

    #[inline]
    pub unsafe fn cmd_end_debug_utils_label_ext(&self,
        command_buffer: VkCommandBuffer) {
        (self.pfn_cmd_end_debug_utils_label_ext)(
            command_buffer,)
    }

    #[inline]
    pub unsafe fn cmd_begin_debug_utils_label_ext(&self,
        command_buffer: VkCommandBuffer,
        p_label_info: *const VkDebugUtilsLabelEXT) {
        (self.pfn_cmd_begin_debug_utils_label_ext)(
            command_buffer,
            p_label_info,)
    }

    #[inline]
    pub unsafe fn queue_insert_debug_utils_label_ext(&self,
        queue: VkQueue,
        p_label_info: *const VkDebugUtilsLabelEXT) {
        (self.pfn_queue_insert_debug_utils_label_ext)(
            queue,
            p_label_info,)
    }

    #[inline]
    pub unsafe fn queue_end_debug_utils_label_ext(&self,
        queue: VkQueue) {
        (self.pfn_queue_end_debug_utils_label_ext)(
            queue,)
    }

    #[inline]
    pub unsafe fn queue_begin_debug_utils_label_ext(&self,
        queue: VkQueue,
        p_label_info: *const VkDebugUtilsLabelEXT) {
        (self.pfn_queue_begin_debug_utils_label_ext)(
            queue,
            p_label_info,)
    }

    #[inline]
    pub unsafe fn set_debug_utils_object_tag_ext(&self,
        device: VkDevice,
        p_tag_info: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult {
        (self.pfn_set_debug_utils_object_tag_ext)(
            device,
            p_tag_info,)
    }

    #[inline]
    pub unsafe fn set_debug_utils_object_name_ext(&self,
        device: VkDevice,
        p_name_info: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult {
        (self.pfn_set_debug_utils_object_name_ext)(
            device,
            p_name_info,)
    }

    #[inline]
    pub unsafe fn get_physical_device_sparse_image_format_properties(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        kind: VkImageType,
        samples: VkSampleCountFlagBits,
        usage: VkImageUsageFlags,
        tiling: VkImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut VkSparseImageFormatProperties) {
        (self.pfn_get_physical_device_sparse_image_format_properties)(
            physical_device,
            format,
            kind,
            samples,
            usage,
            tiling,
            p_property_count,
            p_properties,)
    }

    #[inline]
    pub unsafe fn enumerate_device_layer_properties(&self,
        physical_device: VkPhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut VkLayerProperties) -> VkResult {
        (self.pfn_enumerate_device_layer_properties)(
            physical_device,
            p_property_count,
            p_properties,)
    }

    #[inline]
    pub unsafe fn enumerate_device_extension_properties(&self,
        physical_device: VkPhysicalDevice,
        p_layer_name: *const u8,
        p_property_count: *mut u32,
        p_properties: *mut VkExtensionProperties) -> VkResult {
        (self.pfn_enumerate_device_extension_properties)(
            physical_device,
            p_layer_name,
            p_property_count,
            p_properties,)
    }

    #[inline]
    pub unsafe fn create_device(&self,
        physical_device: VkPhysicalDevice,
        p_create_info: *const VkDeviceCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_device: *mut VkDevice) -> VkResult {
        (self.pfn_create_device)(
            physical_device,
            p_create_info,
            p_allocator,
            p_device,)
    }

    #[inline]
    pub unsafe fn get_device_proc_addr(&self,
        device: VkDevice,
        p_name: *const u8) -> PfnVkVoidFunction {
        (self.pfn_get_device_proc_addr)(
            device,
            p_name,)
    }

    #[inline]
    pub unsafe fn get_physical_device_memory_properties(&self,
        physical_device: VkPhysicalDevice,
        p_memory_properties: *mut VkPhysicalDeviceMemoryProperties) {
        (self.pfn_get_physical_device_memory_properties)(
            physical_device,
            p_memory_properties,)
    }

    #[inline]
    pub unsafe fn get_physical_device_queue_family_properties(&self,
        physical_device: VkPhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut VkQueueFamilyProperties) {
        (self.pfn_get_physical_device_queue_family_properties)(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,)
    }

    #[inline]
    pub unsafe fn get_physical_device_properties(&self,
        physical_device: VkPhysicalDevice,
        p_properties: *mut VkPhysicalDeviceProperties) {
        (self.pfn_get_physical_device_properties)(
            physical_device,
            p_properties,)
    }

    #[inline]
    pub unsafe fn get_physical_device_image_format_properties(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        kind: VkImageType,
        tiling: VkImageTiling,
        usage: VkImageUsageFlags,
        flags: VkImageCreateFlags,
        p_image_format_properties: *mut VkImageFormatProperties) -> VkResult {
        (self.pfn_get_physical_device_image_format_properties)(
            physical_device,
            format,
            kind,
            tiling,
            usage,
            flags,
            p_image_format_properties,)
    }

    #[inline]
    pub unsafe fn get_physical_device_format_properties(&self,
        physical_device: VkPhysicalDevice,
        format: VkFormat,
        p_format_properties: *mut VkFormatProperties) {
        (self.pfn_get_physical_device_format_properties)(
            physical_device,
            format,
            p_format_properties,)
    }

    #[inline]
    pub unsafe fn get_physical_device_features(&self,
        physical_device: VkPhysicalDevice,
        p_features: *mut VkPhysicalDeviceFeatures) {
        (self.pfn_get_physical_device_features)(
            physical_device,
            p_features,)
    }

    #[inline]
    pub unsafe fn enumerate_physical_devices(&self,
        instance: VkInstance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut VkPhysicalDevice) -> VkResult {
        (self.pfn_enumerate_physical_devices)(
            instance,
            p_physical_device_count,
            p_physical_devices,)
    }

    #[inline]
    pub unsafe fn destroy_instance(&self,
        instance: VkInstance,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_instance)(
            instance,
            p_allocator,)
    }
}

#[derive(Clone)]
pub struct DeviceCommands {
    pfn_queue_present_khr: PfnVkQueuePresentKHR,
    pfn_acquire_next_image_khr: PfnVkAcquireNextImageKHR,
    pfn_get_swapchain_images_khr: PfnVkGetSwapchainImagesKHR,
    pfn_destroy_swapchain_khr: PfnVkDestroySwapchainKHR,
    pfn_create_swapchain_khr: PfnVkCreateSwapchainKHR,
    pfn_cmd_execute_commands: PfnVkCmdExecuteCommands,
    pfn_cmd_end_render_pass: PfnVkCmdEndRenderPass,
    pfn_cmd_next_subpass: PfnVkCmdNextSubpass,
    pfn_cmd_begin_render_pass: PfnVkCmdBeginRenderPass,
    pfn_cmd_push_constants: PfnVkCmdPushConstants,
    pfn_cmd_copy_query_pool_results: PfnVkCmdCopyQueryPoolResults,
    pfn_cmd_write_timestamp: PfnVkCmdWriteTimestamp,
    pfn_cmd_reset_query_pool: PfnVkCmdResetQueryPool,
    pfn_cmd_end_query: PfnVkCmdEndQuery,
    pfn_cmd_begin_query: PfnVkCmdBeginQuery,
    pfn_cmd_pipeline_barrier: PfnVkCmdPipelineBarrier,
    pfn_cmd_wait_events: PfnVkCmdWaitEvents,
    pfn_cmd_reset_event: PfnVkCmdResetEvent,
    pfn_cmd_set_event: PfnVkCmdSetEvent,
    pfn_cmd_resolve_image: PfnVkCmdResolveImage,
    pfn_cmd_clear_attachments: PfnVkCmdClearAttachments,
    pfn_cmd_clear_depth_stencil_image: PfnVkCmdClearDepthStencilImage,
    pfn_cmd_clear_color_image: PfnVkCmdClearColorImage,
    pfn_cmd_fill_buffer: PfnVkCmdFillBuffer,
    pfn_cmd_update_buffer: PfnVkCmdUpdateBuffer,
    pfn_cmd_copy_image_to_buffer: PfnVkCmdCopyImageToBuffer,
    pfn_cmd_copy_buffer_to_image: PfnVkCmdCopyBufferToImage,
    pfn_cmd_blit_image: PfnVkCmdBlitImage,
    pfn_cmd_copy_image: PfnVkCmdCopyImage,
    pfn_cmd_copy_buffer: PfnVkCmdCopyBuffer,
    pfn_cmd_dispatch_indirect: PfnVkCmdDispatchIndirect,
    pfn_cmd_dispatch: PfnVkCmdDispatch,
    pfn_cmd_draw_indexed_indirect: PfnVkCmdDrawIndexedIndirect,
    pfn_cmd_draw_indirect: PfnVkCmdDrawIndirect,
    pfn_cmd_draw_indexed: PfnVkCmdDrawIndexed,
    pfn_cmd_draw: PfnVkCmdDraw,
    pfn_cmd_bind_vertex_buffers: PfnVkCmdBindVertexBuffers,
    pfn_cmd_bind_index_buffer: PfnVkCmdBindIndexBuffer,
    pfn_cmd_bind_descriptor_sets: PfnVkCmdBindDescriptorSets,
    pfn_cmd_set_stencil_reference: PfnVkCmdSetStencilReference,
    pfn_cmd_set_stencil_write_mask: PfnVkCmdSetStencilWriteMask,
    pfn_cmd_set_stencil_compare_mask: PfnVkCmdSetStencilCompareMask,
    pfn_cmd_set_depth_bounds: PfnVkCmdSetDepthBounds,
    pfn_cmd_set_blend_constants: PfnVkCmdSetBlendConstants,
    pfn_cmd_set_depth_bias: PfnVkCmdSetDepthBias,
    pfn_cmd_set_line_width: PfnVkCmdSetLineWidth,
    pfn_cmd_set_scissor: PfnVkCmdSetScissor,
    pfn_cmd_set_viewport: PfnVkCmdSetViewport,
    pfn_cmd_bind_pipeline: PfnVkCmdBindPipeline,
    pfn_reset_command_buffer: PfnVkResetCommandBuffer,
    pfn_end_command_buffer: PfnVkEndCommandBuffer,
    pfn_begin_command_buffer: PfnVkBeginCommandBuffer,
    pfn_free_command_buffers: PfnVkFreeCommandBuffers,
    pfn_allocate_command_buffers: PfnVkAllocateCommandBuffers,
    pfn_reset_command_pool: PfnVkResetCommandPool,
    pfn_destroy_command_pool: PfnVkDestroyCommandPool,
    pfn_create_command_pool: PfnVkCreateCommandPool,
    pfn_get_render_area_granularity: PfnVkGetRenderAreaGranularity,
    pfn_destroy_render_pass: PfnVkDestroyRenderPass,
    pfn_create_render_pass: PfnVkCreateRenderPass,
    pfn_destroy_framebuffer: PfnVkDestroyFramebuffer,
    pfn_create_framebuffer: PfnVkCreateFramebuffer,
    pfn_update_descriptor_sets: PfnVkUpdateDescriptorSets,
    pfn_free_descriptor_sets: PfnVkFreeDescriptorSets,
    pfn_allocate_descriptor_sets: PfnVkAllocateDescriptorSets,
    pfn_reset_descriptor_pool: PfnVkResetDescriptorPool,
    pfn_destroy_descriptor_pool: PfnVkDestroyDescriptorPool,
    pfn_create_descriptor_pool: PfnVkCreateDescriptorPool,
    pfn_destroy_descriptor_set_layout: PfnVkDestroyDescriptorSetLayout,
    pfn_create_descriptor_set_layout: PfnVkCreateDescriptorSetLayout,
    pfn_destroy_sampler: PfnVkDestroySampler,
    pfn_create_sampler: PfnVkCreateSampler,
    pfn_destroy_pipeline_layout: PfnVkDestroyPipelineLayout,
    pfn_create_pipeline_layout: PfnVkCreatePipelineLayout,
    pfn_destroy_pipeline: PfnVkDestroyPipeline,
    pfn_create_compute_pipelines: PfnVkCreateComputePipelines,
    pfn_create_graphics_pipelines: PfnVkCreateGraphicsPipelines,
    pfn_merge_pipeline_caches: PfnVkMergePipelineCaches,
    pfn_get_pipeline_cache_data: PfnVkGetPipelineCacheData,
    pfn_destroy_pipeline_cache: PfnVkDestroyPipelineCache,
    pfn_create_pipeline_cache: PfnVkCreatePipelineCache,
    pfn_destroy_shader_module: PfnVkDestroyShaderModule,
    pfn_create_shader_module: PfnVkCreateShaderModule,
    pfn_destroy_image_view: PfnVkDestroyImageView,
    pfn_create_image_view: PfnVkCreateImageView,
    pfn_get_image_subresource_layout: PfnVkGetImageSubresourceLayout,
    pfn_destroy_image: PfnVkDestroyImage,
    pfn_create_image: PfnVkCreateImage,
    pfn_destroy_buffer_view: PfnVkDestroyBufferView,
    pfn_create_buffer_view: PfnVkCreateBufferView,
    pfn_destroy_buffer: PfnVkDestroyBuffer,
    pfn_create_buffer: PfnVkCreateBuffer,
    pfn_get_query_pool_results: PfnVkGetQueryPoolResults,
    pfn_destroy_query_pool: PfnVkDestroyQueryPool,
    pfn_create_query_pool: PfnVkCreateQueryPool,
    pfn_reset_event: PfnVkResetEvent,
    pfn_set_event: PfnVkSetEvent,
    pfn_get_event_status: PfnVkGetEventStatus,
    pfn_destroy_event: PfnVkDestroyEvent,
    pfn_create_event: PfnVkCreateEvent,
    pfn_destroy_semaphore: PfnVkDestroySemaphore,
    pfn_create_semaphore: PfnVkCreateSemaphore,
    pfn_wait_for_fences: PfnVkWaitForFences,
    pfn_get_fence_status: PfnVkGetFenceStatus,
    pfn_reset_fences: PfnVkResetFences,
    pfn_destroy_fence: PfnVkDestroyFence,
    pfn_create_fence: PfnVkCreateFence,
    pfn_queue_bind_sparse: PfnVkQueueBindSparse,
    pfn_get_image_sparse_memory_requirements: PfnVkGetImageSparseMemoryRequirements,
    pfn_get_image_memory_requirements: PfnVkGetImageMemoryRequirements,
    pfn_get_buffer_memory_requirements: PfnVkGetBufferMemoryRequirements,
    pfn_bind_image_memory: PfnVkBindImageMemory,
    pfn_bind_buffer_memory: PfnVkBindBufferMemory,
    pfn_get_device_memory_commitment: PfnVkGetDeviceMemoryCommitment,
    pfn_invalidate_mapped_memory_ranges: PfnVkInvalidateMappedMemoryRanges,
    pfn_flush_mapped_memory_ranges: PfnVkFlushMappedMemoryRanges,
    pfn_unmap_memory: PfnVkUnmapMemory,
    pfn_map_memory: PfnVkMapMemory,
    pfn_free_memory: PfnVkFreeMemory,
    pfn_allocate_memory: PfnVkAllocateMemory,
    pfn_device_wait_idle: PfnVkDeviceWaitIdle,
    pfn_queue_wait_idle: PfnVkQueueWaitIdle,
    pfn_queue_submit: PfnVkQueueSubmit,
    pfn_get_device_queue: PfnVkGetDeviceQueue,
    pfn_destroy_device: PfnVkDestroyDevice,
}

impl DeviceCommands {
    pub fn load(load_fn: impl Fn(&[u8]) -> PfnVkVoidFunction) -> Self {
        DeviceCommands {
            pfn_queue_present_khr: unsafe { core::mem::transmute(load_fn(b"vkQueuePresentKHR\0")) },
            pfn_acquire_next_image_khr: unsafe { core::mem::transmute(load_fn(b"vkAcquireNextImageKHR\0")) },
            pfn_get_swapchain_images_khr: unsafe { core::mem::transmute(load_fn(b"vkGetSwapchainImagesKHR\0")) },
            pfn_destroy_swapchain_khr: unsafe { core::mem::transmute(load_fn(b"vkDestroySwapchainKHR\0")) },
            pfn_create_swapchain_khr: unsafe { core::mem::transmute(load_fn(b"vkCreateSwapchainKHR\0")) },
            pfn_cmd_execute_commands: unsafe { core::mem::transmute(load_fn(b"vkCmdExecuteCommands\0")) },
            pfn_cmd_end_render_pass: unsafe { core::mem::transmute(load_fn(b"vkCmdEndRenderPass\0")) },
            pfn_cmd_next_subpass: unsafe { core::mem::transmute(load_fn(b"vkCmdNextSubpass\0")) },
            pfn_cmd_begin_render_pass: unsafe { core::mem::transmute(load_fn(b"vkCmdBeginRenderPass\0")) },
            pfn_cmd_push_constants: unsafe { core::mem::transmute(load_fn(b"vkCmdPushConstants\0")) },
            pfn_cmd_copy_query_pool_results: unsafe { core::mem::transmute(load_fn(b"vkCmdCopyQueryPoolResults\0")) },
            pfn_cmd_write_timestamp: unsafe { core::mem::transmute(load_fn(b"vkCmdWriteTimestamp\0")) },
            pfn_cmd_reset_query_pool: unsafe { core::mem::transmute(load_fn(b"vkCmdResetQueryPool\0")) },
            pfn_cmd_end_query: unsafe { core::mem::transmute(load_fn(b"vkCmdEndQuery\0")) },
            pfn_cmd_begin_query: unsafe { core::mem::transmute(load_fn(b"vkCmdBeginQuery\0")) },
            pfn_cmd_pipeline_barrier: unsafe { core::mem::transmute(load_fn(b"vkCmdPipelineBarrier\0")) },
            pfn_cmd_wait_events: unsafe { core::mem::transmute(load_fn(b"vkCmdWaitEvents\0")) },
            pfn_cmd_reset_event: unsafe { core::mem::transmute(load_fn(b"vkCmdResetEvent\0")) },
            pfn_cmd_set_event: unsafe { core::mem::transmute(load_fn(b"vkCmdSetEvent\0")) },
            pfn_cmd_resolve_image: unsafe { core::mem::transmute(load_fn(b"vkCmdResolveImage\0")) },
            pfn_cmd_clear_attachments: unsafe { core::mem::transmute(load_fn(b"vkCmdClearAttachments\0")) },
            pfn_cmd_clear_depth_stencil_image: unsafe { core::mem::transmute(load_fn(b"vkCmdClearDepthStencilImage\0")) },
            pfn_cmd_clear_color_image: unsafe { core::mem::transmute(load_fn(b"vkCmdClearColorImage\0")) },
            pfn_cmd_fill_buffer: unsafe { core::mem::transmute(load_fn(b"vkCmdFillBuffer\0")) },
            pfn_cmd_update_buffer: unsafe { core::mem::transmute(load_fn(b"vkCmdUpdateBuffer\0")) },
            pfn_cmd_copy_image_to_buffer: unsafe { core::mem::transmute(load_fn(b"vkCmdCopyImageToBuffer\0")) },
            pfn_cmd_copy_buffer_to_image: unsafe { core::mem::transmute(load_fn(b"vkCmdCopyBufferToImage\0")) },
            pfn_cmd_blit_image: unsafe { core::mem::transmute(load_fn(b"vkCmdBlitImage\0")) },
            pfn_cmd_copy_image: unsafe { core::mem::transmute(load_fn(b"vkCmdCopyImage\0")) },
            pfn_cmd_copy_buffer: unsafe { core::mem::transmute(load_fn(b"vkCmdCopyBuffer\0")) },
            pfn_cmd_dispatch_indirect: unsafe { core::mem::transmute(load_fn(b"vkCmdDispatchIndirect\0")) },
            pfn_cmd_dispatch: unsafe { core::mem::transmute(load_fn(b"vkCmdDispatch\0")) },
            pfn_cmd_draw_indexed_indirect: unsafe { core::mem::transmute(load_fn(b"vkCmdDrawIndexedIndirect\0")) },
            pfn_cmd_draw_indirect: unsafe { core::mem::transmute(load_fn(b"vkCmdDrawIndirect\0")) },
            pfn_cmd_draw_indexed: unsafe { core::mem::transmute(load_fn(b"vkCmdDrawIndexed\0")) },
            pfn_cmd_draw: unsafe { core::mem::transmute(load_fn(b"vkCmdDraw\0")) },
            pfn_cmd_bind_vertex_buffers: unsafe { core::mem::transmute(load_fn(b"vkCmdBindVertexBuffers\0")) },
            pfn_cmd_bind_index_buffer: unsafe { core::mem::transmute(load_fn(b"vkCmdBindIndexBuffer\0")) },
            pfn_cmd_bind_descriptor_sets: unsafe { core::mem::transmute(load_fn(b"vkCmdBindDescriptorSets\0")) },
            pfn_cmd_set_stencil_reference: unsafe { core::mem::transmute(load_fn(b"vkCmdSetStencilReference\0")) },
            pfn_cmd_set_stencil_write_mask: unsafe { core::mem::transmute(load_fn(b"vkCmdSetStencilWriteMask\0")) },
            pfn_cmd_set_stencil_compare_mask: unsafe { core::mem::transmute(load_fn(b"vkCmdSetStencilCompareMask\0")) },
            pfn_cmd_set_depth_bounds: unsafe { core::mem::transmute(load_fn(b"vkCmdSetDepthBounds\0")) },
            pfn_cmd_set_blend_constants: unsafe { core::mem::transmute(load_fn(b"vkCmdSetBlendConstants\0")) },
            pfn_cmd_set_depth_bias: unsafe { core::mem::transmute(load_fn(b"vkCmdSetDepthBias\0")) },
            pfn_cmd_set_line_width: unsafe { core::mem::transmute(load_fn(b"vkCmdSetLineWidth\0")) },
            pfn_cmd_set_scissor: unsafe { core::mem::transmute(load_fn(b"vkCmdSetScissor\0")) },
            pfn_cmd_set_viewport: unsafe { core::mem::transmute(load_fn(b"vkCmdSetViewport\0")) },
            pfn_cmd_bind_pipeline: unsafe { core::mem::transmute(load_fn(b"vkCmdBindPipeline\0")) },
            pfn_reset_command_buffer: unsafe { core::mem::transmute(load_fn(b"vkResetCommandBuffer\0")) },
            pfn_end_command_buffer: unsafe { core::mem::transmute(load_fn(b"vkEndCommandBuffer\0")) },
            pfn_begin_command_buffer: unsafe { core::mem::transmute(load_fn(b"vkBeginCommandBuffer\0")) },
            pfn_free_command_buffers: unsafe { core::mem::transmute(load_fn(b"vkFreeCommandBuffers\0")) },
            pfn_allocate_command_buffers: unsafe { core::mem::transmute(load_fn(b"vkAllocateCommandBuffers\0")) },
            pfn_reset_command_pool: unsafe { core::mem::transmute(load_fn(b"vkResetCommandPool\0")) },
            pfn_destroy_command_pool: unsafe { core::mem::transmute(load_fn(b"vkDestroyCommandPool\0")) },
            pfn_create_command_pool: unsafe { core::mem::transmute(load_fn(b"vkCreateCommandPool\0")) },
            pfn_get_render_area_granularity: unsafe { core::mem::transmute(load_fn(b"vkGetRenderAreaGranularity\0")) },
            pfn_destroy_render_pass: unsafe { core::mem::transmute(load_fn(b"vkDestroyRenderPass\0")) },
            pfn_create_render_pass: unsafe { core::mem::transmute(load_fn(b"vkCreateRenderPass\0")) },
            pfn_destroy_framebuffer: unsafe { core::mem::transmute(load_fn(b"vkDestroyFramebuffer\0")) },
            pfn_create_framebuffer: unsafe { core::mem::transmute(load_fn(b"vkCreateFramebuffer\0")) },
            pfn_update_descriptor_sets: unsafe { core::mem::transmute(load_fn(b"vkUpdateDescriptorSets\0")) },
            pfn_free_descriptor_sets: unsafe { core::mem::transmute(load_fn(b"vkFreeDescriptorSets\0")) },
            pfn_allocate_descriptor_sets: unsafe { core::mem::transmute(load_fn(b"vkAllocateDescriptorSets\0")) },
            pfn_reset_descriptor_pool: unsafe { core::mem::transmute(load_fn(b"vkResetDescriptorPool\0")) },
            pfn_destroy_descriptor_pool: unsafe { core::mem::transmute(load_fn(b"vkDestroyDescriptorPool\0")) },
            pfn_create_descriptor_pool: unsafe { core::mem::transmute(load_fn(b"vkCreateDescriptorPool\0")) },
            pfn_destroy_descriptor_set_layout: unsafe { core::mem::transmute(load_fn(b"vkDestroyDescriptorSetLayout\0")) },
            pfn_create_descriptor_set_layout: unsafe { core::mem::transmute(load_fn(b"vkCreateDescriptorSetLayout\0")) },
            pfn_destroy_sampler: unsafe { core::mem::transmute(load_fn(b"vkDestroySampler\0")) },
            pfn_create_sampler: unsafe { core::mem::transmute(load_fn(b"vkCreateSampler\0")) },
            pfn_destroy_pipeline_layout: unsafe { core::mem::transmute(load_fn(b"vkDestroyPipelineLayout\0")) },
            pfn_create_pipeline_layout: unsafe { core::mem::transmute(load_fn(b"vkCreatePipelineLayout\0")) },
            pfn_destroy_pipeline: unsafe { core::mem::transmute(load_fn(b"vkDestroyPipeline\0")) },
            pfn_create_compute_pipelines: unsafe { core::mem::transmute(load_fn(b"vkCreateComputePipelines\0")) },
            pfn_create_graphics_pipelines: unsafe { core::mem::transmute(load_fn(b"vkCreateGraphicsPipelines\0")) },
            pfn_merge_pipeline_caches: unsafe { core::mem::transmute(load_fn(b"vkMergePipelineCaches\0")) },
            pfn_get_pipeline_cache_data: unsafe { core::mem::transmute(load_fn(b"vkGetPipelineCacheData\0")) },
            pfn_destroy_pipeline_cache: unsafe { core::mem::transmute(load_fn(b"vkDestroyPipelineCache\0")) },
            pfn_create_pipeline_cache: unsafe { core::mem::transmute(load_fn(b"vkCreatePipelineCache\0")) },
            pfn_destroy_shader_module: unsafe { core::mem::transmute(load_fn(b"vkDestroyShaderModule\0")) },
            pfn_create_shader_module: unsafe { core::mem::transmute(load_fn(b"vkCreateShaderModule\0")) },
            pfn_destroy_image_view: unsafe { core::mem::transmute(load_fn(b"vkDestroyImageView\0")) },
            pfn_create_image_view: unsafe { core::mem::transmute(load_fn(b"vkCreateImageView\0")) },
            pfn_get_image_subresource_layout: unsafe { core::mem::transmute(load_fn(b"vkGetImageSubresourceLayout\0")) },
            pfn_destroy_image: unsafe { core::mem::transmute(load_fn(b"vkDestroyImage\0")) },
            pfn_create_image: unsafe { core::mem::transmute(load_fn(b"vkCreateImage\0")) },
            pfn_destroy_buffer_view: unsafe { core::mem::transmute(load_fn(b"vkDestroyBufferView\0")) },
            pfn_create_buffer_view: unsafe { core::mem::transmute(load_fn(b"vkCreateBufferView\0")) },
            pfn_destroy_buffer: unsafe { core::mem::transmute(load_fn(b"vkDestroyBuffer\0")) },
            pfn_create_buffer: unsafe { core::mem::transmute(load_fn(b"vkCreateBuffer\0")) },
            pfn_get_query_pool_results: unsafe { core::mem::transmute(load_fn(b"vkGetQueryPoolResults\0")) },
            pfn_destroy_query_pool: unsafe { core::mem::transmute(load_fn(b"vkDestroyQueryPool\0")) },
            pfn_create_query_pool: unsafe { core::mem::transmute(load_fn(b"vkCreateQueryPool\0")) },
            pfn_reset_event: unsafe { core::mem::transmute(load_fn(b"vkResetEvent\0")) },
            pfn_set_event: unsafe { core::mem::transmute(load_fn(b"vkSetEvent\0")) },
            pfn_get_event_status: unsafe { core::mem::transmute(load_fn(b"vkGetEventStatus\0")) },
            pfn_destroy_event: unsafe { core::mem::transmute(load_fn(b"vkDestroyEvent\0")) },
            pfn_create_event: unsafe { core::mem::transmute(load_fn(b"vkCreateEvent\0")) },
            pfn_destroy_semaphore: unsafe { core::mem::transmute(load_fn(b"vkDestroySemaphore\0")) },
            pfn_create_semaphore: unsafe { core::mem::transmute(load_fn(b"vkCreateSemaphore\0")) },
            pfn_wait_for_fences: unsafe { core::mem::transmute(load_fn(b"vkWaitForFences\0")) },
            pfn_get_fence_status: unsafe { core::mem::transmute(load_fn(b"vkGetFenceStatus\0")) },
            pfn_reset_fences: unsafe { core::mem::transmute(load_fn(b"vkResetFences\0")) },
            pfn_destroy_fence: unsafe { core::mem::transmute(load_fn(b"vkDestroyFence\0")) },
            pfn_create_fence: unsafe { core::mem::transmute(load_fn(b"vkCreateFence\0")) },
            pfn_queue_bind_sparse: unsafe { core::mem::transmute(load_fn(b"vkQueueBindSparse\0")) },
            pfn_get_image_sparse_memory_requirements: unsafe { core::mem::transmute(load_fn(b"vkGetImageSparseMemoryRequirements\0")) },
            pfn_get_image_memory_requirements: unsafe { core::mem::transmute(load_fn(b"vkGetImageMemoryRequirements\0")) },
            pfn_get_buffer_memory_requirements: unsafe { core::mem::transmute(load_fn(b"vkGetBufferMemoryRequirements\0")) },
            pfn_bind_image_memory: unsafe { core::mem::transmute(load_fn(b"vkBindImageMemory\0")) },
            pfn_bind_buffer_memory: unsafe { core::mem::transmute(load_fn(b"vkBindBufferMemory\0")) },
            pfn_get_device_memory_commitment: unsafe { core::mem::transmute(load_fn(b"vkGetDeviceMemoryCommitment\0")) },
            pfn_invalidate_mapped_memory_ranges: unsafe { core::mem::transmute(load_fn(b"vkInvalidateMappedMemoryRanges\0")) },
            pfn_flush_mapped_memory_ranges: unsafe { core::mem::transmute(load_fn(b"vkFlushMappedMemoryRanges\0")) },
            pfn_unmap_memory: unsafe { core::mem::transmute(load_fn(b"vkUnmapMemory\0")) },
            pfn_map_memory: unsafe { core::mem::transmute(load_fn(b"vkMapMemory\0")) },
            pfn_free_memory: unsafe { core::mem::transmute(load_fn(b"vkFreeMemory\0")) },
            pfn_allocate_memory: unsafe { core::mem::transmute(load_fn(b"vkAllocateMemory\0")) },
            pfn_device_wait_idle: unsafe { core::mem::transmute(load_fn(b"vkDeviceWaitIdle\0")) },
            pfn_queue_wait_idle: unsafe { core::mem::transmute(load_fn(b"vkQueueWaitIdle\0")) },
            pfn_queue_submit: unsafe { core::mem::transmute(load_fn(b"vkQueueSubmit\0")) },
            pfn_get_device_queue: unsafe { core::mem::transmute(load_fn(b"vkGetDeviceQueue\0")) },
            pfn_destroy_device: unsafe { core::mem::transmute(load_fn(b"vkDestroyDevice\0")) },
        }
    }

    #[inline]
    pub unsafe fn queue_present_khr(&self,
        queue: VkQueue,
        p_present_info: *const VkPresentInfoKHR) -> VkResult {
        (self.pfn_queue_present_khr)(
            queue,
            p_present_info,)
    }

    #[inline]
    pub unsafe fn acquire_next_image_khr(&self,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence,
        p_image_index: *mut u32) -> VkResult {
        (self.pfn_acquire_next_image_khr)(
            device,
            swapchain,
            timeout,
            semaphore,
            fence,
            p_image_index,)
    }

    #[inline]
    pub unsafe fn get_swapchain_images_khr(&self,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut VkImage) -> VkResult {
        (self.pfn_get_swapchain_images_khr)(
            device,
            swapchain,
            p_swapchain_image_count,
            p_swapchain_images,)
    }

    #[inline]
    pub unsafe fn destroy_swapchain_khr(&self,
        device: VkDevice,
        swapchain: VkSwapchainKHR,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_swapchain_khr)(
            device,
            swapchain,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_swapchain_khr(&self,
        device: VkDevice,
        p_create_info: *const VkSwapchainCreateInfoKHR,
        p_allocator: *const VkAllocationCallbacks,
        p_swapchain: *mut VkSwapchainKHR) -> VkResult {
        (self.pfn_create_swapchain_khr)(
            device,
            p_create_info,
            p_allocator,
            p_swapchain,)
    }

    #[inline]
    pub unsafe fn cmd_execute_commands(&self,
        command_buffer: VkCommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const VkCommandBuffer) {
        (self.pfn_cmd_execute_commands)(
            command_buffer,
            command_buffer_count,
            p_command_buffers,)
    }

    #[inline]
    pub unsafe fn cmd_end_render_pass(&self,
        command_buffer: VkCommandBuffer) {
        (self.pfn_cmd_end_render_pass)(
            command_buffer,)
    }

    #[inline]
    pub unsafe fn cmd_next_subpass(&self,
        command_buffer: VkCommandBuffer,
        contents: VkSubpassContents) {
        (self.pfn_cmd_next_subpass)(
            command_buffer,
            contents,)
    }

    #[inline]
    pub unsafe fn cmd_begin_render_pass(&self,
        command_buffer: VkCommandBuffer,
        p_render_pass_begin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents) {
        (self.pfn_cmd_begin_render_pass)(
            command_buffer,
            p_render_pass_begin,
            contents,)
    }

    #[inline]
    pub unsafe fn cmd_push_constants(&self,
        command_buffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stage_flags: VkShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const core::ffi::c_void) {
        (self.pfn_cmd_push_constants)(
            command_buffer,
            layout,
            stage_flags,
            offset,
            size,
            p_values,)
    }

    #[inline]
    pub unsafe fn cmd_copy_query_pool_results(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: VkBuffer,
        dst_offset: VkDeviceSize,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags) {
        (self.pfn_cmd_copy_query_pool_results)(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,)
    }

    #[inline]
    pub unsafe fn cmd_write_timestamp(&self,
        command_buffer: VkCommandBuffer,
        pipeline_stage: VkPipelineStageFlagBits,
        query_pool: VkQueryPool,
        query: u32) {
        (self.pfn_cmd_write_timestamp)(
            command_buffer,
            pipeline_stage,
            query_pool,
            query,)
    }

    #[inline]
    pub unsafe fn cmd_reset_query_pool(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32) {
        (self.pfn_cmd_reset_query_pool)(
            command_buffer,
            query_pool,
            first_query,
            query_count,)
    }

    #[inline]
    pub unsafe fn cmd_end_query(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        query: u32) {
        (self.pfn_cmd_end_query)(
            command_buffer,
            query_pool,
            query,)
    }

    #[inline]
    pub unsafe fn cmd_begin_query(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        query: u32,
        flags: VkQueryControlFlags) {
        (self.pfn_cmd_begin_query)(
            command_buffer,
            query_pool,
            query,
            flags,)
    }

    #[inline]
    pub unsafe fn cmd_pipeline_barrier(&self,
        command_buffer: VkCommandBuffer,
        src_stage_mask: VkPipelineStageFlags,
        dst_stage_mask: VkPipelineStageFlags,
        dependency_flags: VkDependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const VkMemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const VkBufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const VkImageMemoryBarrier) {
        (self.pfn_cmd_pipeline_barrier)(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barrier_count,
            p_memory_barriers,
            buffer_memory_barrier_count,
            p_buffer_memory_barriers,
            image_memory_barrier_count,
            p_image_memory_barriers,)
    }

    #[inline]
    pub unsafe fn cmd_wait_events(&self,
        command_buffer: VkCommandBuffer,
        event_count: u32,
        p_events: *const VkEvent,
        src_stage_mask: VkPipelineStageFlags,
        dst_stage_mask: VkPipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const VkMemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const VkBufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const VkImageMemoryBarrier) {
        (self.pfn_cmd_wait_events)(
            command_buffer,
            event_count,
            p_events,
            src_stage_mask,
            dst_stage_mask,
            memory_barrier_count,
            p_memory_barriers,
            buffer_memory_barrier_count,
            p_buffer_memory_barriers,
            image_memory_barrier_count,
            p_image_memory_barriers,)
    }

    #[inline]
    pub unsafe fn cmd_reset_event(&self,
        command_buffer: VkCommandBuffer,
        event: VkEvent,
        stage_mask: VkPipelineStageFlags) {
        (self.pfn_cmd_reset_event)(
            command_buffer,
            event,
            stage_mask,)
    }

    #[inline]
    pub unsafe fn cmd_set_event(&self,
        command_buffer: VkCommandBuffer,
        event: VkEvent,
        stage_mask: VkPipelineStageFlags) {
        (self.pfn_cmd_set_event)(
            command_buffer,
            event,
            stage_mask,)
    }

    #[inline]
    pub unsafe fn cmd_resolve_image(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        region_count: u32,
        p_regions: *const VkImageResolve) {
        (self.pfn_cmd_resolve_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,)
    }

    #[inline]
    pub unsafe fn cmd_clear_attachments(&self,
        command_buffer: VkCommandBuffer,
        attachment_count: u32,
        p_attachments: *const VkClearAttachment,
        rect_count: u32,
        p_rects: *const VkClearRect) {
        (self.pfn_cmd_clear_attachments)(
            command_buffer,
            attachment_count,
            p_attachments,
            rect_count,
            p_rects,)
    }

    #[inline]
    pub unsafe fn cmd_clear_depth_stencil_image(&self,
        command_buffer: VkCommandBuffer,
        image: VkImage,
        image_layout: VkImageLayout,
        p_depth_stencil: *const VkClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const VkImageSubresourceRange) {
        (self.pfn_cmd_clear_depth_stencil_image)(
            command_buffer,
            image,
            image_layout,
            p_depth_stencil,
            range_count,
            p_ranges,)
    }

    #[inline]
    pub unsafe fn cmd_clear_color_image(&self,
        command_buffer: VkCommandBuffer,
        image: VkImage,
        image_layout: VkImageLayout,
        p_color: *const VkClearColorValue,
        range_count: u32,
        p_ranges: *const VkImageSubresourceRange) {
        (self.pfn_cmd_clear_color_image)(
            command_buffer,
            image,
            image_layout,
            p_color,
            range_count,
            p_ranges,)
    }

    #[inline]
    pub unsafe fn cmd_fill_buffer(&self,
        command_buffer: VkCommandBuffer,
        dst_buffer: VkBuffer,
        dst_offset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32) {
        (self.pfn_cmd_fill_buffer)(
            command_buffer,
            dst_buffer,
            dst_offset,
            size,
            data,)
    }

    #[inline]
    pub unsafe fn cmd_update_buffer(&self,
        command_buffer: VkCommandBuffer,
        dst_buffer: VkBuffer,
        dst_offset: VkDeviceSize,
        data_size: VkDeviceSize,
        p_data: *const core::ffi::c_void) {
        (self.pfn_cmd_update_buffer)(
            command_buffer,
            dst_buffer,
            dst_offset,
            data_size,
            p_data,)
    }

    #[inline]
    pub unsafe fn cmd_copy_image_to_buffer(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_buffer: VkBuffer,
        region_count: u32,
        p_regions: *const VkBufferImageCopy) {
        (self.pfn_cmd_copy_image_to_buffer)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            region_count,
            p_regions,)
    }

    #[inline]
    pub unsafe fn cmd_copy_buffer_to_image(&self,
        command_buffer: VkCommandBuffer,
        src_buffer: VkBuffer,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        region_count: u32,
        p_regions: *const VkBufferImageCopy) {
        (self.pfn_cmd_copy_buffer_to_image)(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,)
    }

    #[inline]
    pub unsafe fn cmd_blit_image(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        region_count: u32,
        p_regions: *const VkImageBlit,
        filter: VkFilter) {
        (self.pfn_cmd_blit_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
            filter,)
    }

    #[inline]
    pub unsafe fn cmd_copy_image(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        region_count: u32,
        p_regions: *const VkImageCopy) {
        (self.pfn_cmd_copy_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,)
    }

    #[inline]
    pub unsafe fn cmd_copy_buffer(&self,
        command_buffer: VkCommandBuffer,
        src_buffer: VkBuffer,
        dst_buffer: VkBuffer,
        region_count: u32,
        p_regions: *const VkBufferCopy) {
        (self.pfn_cmd_copy_buffer)(
            command_buffer,
            src_buffer,
            dst_buffer,
            region_count,
            p_regions,)
    }

    #[inline]
    pub unsafe fn cmd_dispatch_indirect(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize) {
        (self.pfn_cmd_dispatch_indirect)(
            command_buffer,
            buffer,
            offset,)
    }

    #[inline]
    pub unsafe fn cmd_dispatch(&self,
        command_buffer: VkCommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32) {
        (self.pfn_cmd_dispatch)(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,)
    }

    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        draw_count: u32,
        stride: u32) {
        (self.pfn_cmd_draw_indexed_indirect)(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,)
    }

    #[inline]
    pub unsafe fn cmd_draw_indirect(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        draw_count: u32,
        stride: u32) {
        (self.pfn_cmd_draw_indirect)(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,)
    }

    #[inline]
    pub unsafe fn cmd_draw_indexed(&self,
        command_buffer: VkCommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32) {
        (self.pfn_cmd_draw_indexed)(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,)
    }

    #[inline]
    pub unsafe fn cmd_draw(&self,
        command_buffer: VkCommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32) {
        (self.pfn_cmd_draw)(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,)
    }

    #[inline]
    pub unsafe fn cmd_bind_vertex_buffers(&self,
        command_buffer: VkCommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const VkBuffer,
        p_offsets: *const VkDeviceSize) {
        (self.pfn_cmd_bind_vertex_buffers)(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,)
    }

    #[inline]
    pub unsafe fn cmd_bind_index_buffer(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        index_type: VkIndexType) {
        (self.pfn_cmd_bind_index_buffer)(
            command_buffer,
            buffer,
            offset,
            index_type,)
    }

    #[inline]
    pub unsafe fn cmd_bind_descriptor_sets(&self,
        command_buffer: VkCommandBuffer,
        pipeline_bind_point: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const VkDescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32) {
        (self.pfn_cmd_bind_descriptor_sets)(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_set_count,
            p_descriptor_sets,
            dynamic_offset_count,
            p_dynamic_offsets,)
    }

    #[inline]
    pub unsafe fn cmd_set_stencil_reference(&self,
        command_buffer: VkCommandBuffer,
        face_mask: VkStencilFaceFlags,
        reference: u32) {
        (self.pfn_cmd_set_stencil_reference)(
            command_buffer,
            face_mask,
            reference,)
    }

    #[inline]
    pub unsafe fn cmd_set_stencil_write_mask(&self,
        command_buffer: VkCommandBuffer,
        face_mask: VkStencilFaceFlags,
        write_mask: u32) {
        (self.pfn_cmd_set_stencil_write_mask)(
            command_buffer,
            face_mask,
            write_mask,)
    }

    #[inline]
    pub unsafe fn cmd_set_stencil_compare_mask(&self,
        command_buffer: VkCommandBuffer,
        face_mask: VkStencilFaceFlags,
        compare_mask: u32) {
        (self.pfn_cmd_set_stencil_compare_mask)(
            command_buffer,
            face_mask,
            compare_mask,)
    }

    #[inline]
    pub unsafe fn cmd_set_depth_bounds(&self,
        command_buffer: VkCommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32) {
        (self.pfn_cmd_set_depth_bounds)(
            command_buffer,
            min_depth_bounds,
            max_depth_bounds,)
    }

    #[inline]
    pub unsafe fn cmd_set_blend_constants(&self,
        command_buffer: VkCommandBuffer,
        blend_constants: [f32; 4]) {
        (self.pfn_cmd_set_blend_constants)(
            command_buffer,
            blend_constants,)
    }

    #[inline]
    pub unsafe fn cmd_set_depth_bias(&self,
        command_buffer: VkCommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32) {
        (self.pfn_cmd_set_depth_bias)(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,)
    }

    #[inline]
    pub unsafe fn cmd_set_line_width(&self,
        command_buffer: VkCommandBuffer,
        line_width: f32) {
        (self.pfn_cmd_set_line_width)(
            command_buffer,
            line_width,)
    }

    #[inline]
    pub unsafe fn cmd_set_scissor(&self,
        command_buffer: VkCommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const VkRect2D) {
        (self.pfn_cmd_set_scissor)(
            command_buffer,
            first_scissor,
            scissor_count,
            p_scissors,)
    }

    #[inline]
    pub unsafe fn cmd_set_viewport(&self,
        command_buffer: VkCommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const VkViewport) {
        (self.pfn_cmd_set_viewport)(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewports,)
    }

    #[inline]
    pub unsafe fn cmd_bind_pipeline(&self,
        command_buffer: VkCommandBuffer,
        pipeline_bind_point: VkPipelineBindPoint,
        pipeline: VkPipeline) {
        (self.pfn_cmd_bind_pipeline)(
            command_buffer,
            pipeline_bind_point,
            pipeline,)
    }

    #[inline]
    pub unsafe fn reset_command_buffer(&self,
        command_buffer: VkCommandBuffer,
        flags: VkCommandBufferResetFlags) -> VkResult {
        (self.pfn_reset_command_buffer)(
            command_buffer,
            flags,)
    }

    #[inline]
    pub unsafe fn end_command_buffer(&self,
        command_buffer: VkCommandBuffer) -> VkResult {
        (self.pfn_end_command_buffer)(
            command_buffer,)
    }

    #[inline]
    pub unsafe fn begin_command_buffer(&self,
        command_buffer: VkCommandBuffer,
        p_begin_info: *const VkCommandBufferBeginInfo) -> VkResult {
        (self.pfn_begin_command_buffer)(
            command_buffer,
            p_begin_info,)
    }

    #[inline]
    pub unsafe fn free_command_buffers(&self,
        device: VkDevice,
        command_pool: VkCommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const VkCommandBuffer) {
        (self.pfn_free_command_buffers)(
            device,
            command_pool,
            command_buffer_count,
            p_command_buffers,)
    }

    #[inline]
    pub unsafe fn allocate_command_buffers(&self,
        device: VkDevice,
        p_allocate_info: *const VkCommandBufferAllocateInfo,
        p_command_buffers: *mut VkCommandBuffer) -> VkResult {
        (self.pfn_allocate_command_buffers)(
            device,
            p_allocate_info,
            p_command_buffers,)
    }

    #[inline]
    pub unsafe fn reset_command_pool(&self,
        device: VkDevice,
        command_pool: VkCommandPool,
        flags: VkCommandPoolResetFlags) -> VkResult {
        (self.pfn_reset_command_pool)(
            device,
            command_pool,
            flags,)
    }

    #[inline]
    pub unsafe fn destroy_command_pool(&self,
        device: VkDevice,
        command_pool: VkCommandPool,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_command_pool)(
            device,
            command_pool,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_command_pool(&self,
        device: VkDevice,
        p_create_info: *const VkCommandPoolCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_command_pool: *mut VkCommandPool) -> VkResult {
        (self.pfn_create_command_pool)(
            device,
            p_create_info,
            p_allocator,
            p_command_pool,)
    }

    #[inline]
    pub unsafe fn get_render_area_granularity(&self,
        device: VkDevice,
        render_pass: VkRenderPass,
        p_granularity: *mut VkExtent2D) {
        (self.pfn_get_render_area_granularity)(
            device,
            render_pass,
            p_granularity,)
    }

    #[inline]
    pub unsafe fn destroy_render_pass(&self,
        device: VkDevice,
        render_pass: VkRenderPass,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_render_pass)(
            device,
            render_pass,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_render_pass(&self,
        device: VkDevice,
        p_create_info: *const VkRenderPassCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_render_pass: *mut VkRenderPass) -> VkResult {
        (self.pfn_create_render_pass)(
            device,
            p_create_info,
            p_allocator,
            p_render_pass,)
    }

    #[inline]
    pub unsafe fn destroy_framebuffer(&self,
        device: VkDevice,
        framebuffer: VkFramebuffer,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_framebuffer)(
            device,
            framebuffer,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_framebuffer(&self,
        device: VkDevice,
        p_create_info: *const VkFramebufferCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_framebuffer: *mut VkFramebuffer) -> VkResult {
        (self.pfn_create_framebuffer)(
            device,
            p_create_info,
            p_allocator,
            p_framebuffer,)
    }

    #[inline]
    pub unsafe fn update_descriptor_sets(&self,
        device: VkDevice,
        descriptor_write_count: u32,
        p_descriptor_writes: *const VkWriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const VkCopyDescriptorSet) {
        (self.pfn_update_descriptor_sets)(
            device,
            descriptor_write_count,
            p_descriptor_writes,
            descriptor_copy_count,
            p_descriptor_copies,)
    }

    #[inline]
    pub unsafe fn free_descriptor_sets(&self,
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const VkDescriptorSet) -> VkResult {
        (self.pfn_free_descriptor_sets)(
            device,
            descriptor_pool,
            descriptor_set_count,
            p_descriptor_sets,)
    }

    #[inline]
    pub unsafe fn allocate_descriptor_sets(&self,
        device: VkDevice,
        p_allocate_info: *const VkDescriptorSetAllocateInfo,
        p_descriptor_sets: *mut VkDescriptorSet) -> VkResult {
        (self.pfn_allocate_descriptor_sets)(
            device,
            p_allocate_info,
            p_descriptor_sets,)
    }

    #[inline]
    pub unsafe fn reset_descriptor_pool(&self,
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        flags: VkDescriptorPoolResetFlags) -> VkResult {
        (self.pfn_reset_descriptor_pool)(
            device,
            descriptor_pool,
            flags,)
    }

    #[inline]
    pub unsafe fn destroy_descriptor_pool(&self,
        device: VkDevice,
        descriptor_pool: VkDescriptorPool,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_descriptor_pool)(
            device,
            descriptor_pool,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_descriptor_pool(&self,
        device: VkDevice,
        p_create_info: *const VkDescriptorPoolCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_descriptor_pool: *mut VkDescriptorPool) -> VkResult {
        (self.pfn_create_descriptor_pool)(
            device,
            p_create_info,
            p_allocator,
            p_descriptor_pool,)
    }

    #[inline]
    pub unsafe fn destroy_descriptor_set_layout(&self,
        device: VkDevice,
        descriptor_set_layout: VkDescriptorSetLayout,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_descriptor_set_layout)(
            device,
            descriptor_set_layout,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_descriptor_set_layout(&self,
        device: VkDevice,
        p_create_info: *const VkDescriptorSetLayoutCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_set_layout: *mut VkDescriptorSetLayout) -> VkResult {
        (self.pfn_create_descriptor_set_layout)(
            device,
            p_create_info,
            p_allocator,
            p_set_layout,)
    }

    #[inline]
    pub unsafe fn destroy_sampler(&self,
        device: VkDevice,
        sampler: VkSampler,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_sampler)(
            device,
            sampler,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_sampler(&self,
        device: VkDevice,
        p_create_info: *const VkSamplerCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_sampler: *mut VkSampler) -> VkResult {
        (self.pfn_create_sampler)(
            device,
            p_create_info,
            p_allocator,
            p_sampler,)
    }

    #[inline]
    pub unsafe fn destroy_pipeline_layout(&self,
        device: VkDevice,
        pipeline_layout: VkPipelineLayout,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_pipeline_layout)(
            device,
            pipeline_layout,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_pipeline_layout(&self,
        device: VkDevice,
        p_create_info: *const VkPipelineLayoutCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_pipeline_layout: *mut VkPipelineLayout) -> VkResult {
        (self.pfn_create_pipeline_layout)(
            device,
            p_create_info,
            p_allocator,
            p_pipeline_layout,)
    }

    #[inline]
    pub unsafe fn destroy_pipeline(&self,
        device: VkDevice,
        pipeline: VkPipeline,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_pipeline)(
            device,
            pipeline,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_compute_pipelines(&self,
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        create_info_count: u32,
        p_create_infos: *const VkComputePipelineCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_pipelines: *mut VkPipeline) -> VkResult {
        (self.pfn_create_compute_pipelines)(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,)
    }

    #[inline]
    pub unsafe fn create_graphics_pipelines(&self,
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        create_info_count: u32,
        p_create_infos: *const VkGraphicsPipelineCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_pipelines: *mut VkPipeline) -> VkResult {
        (self.pfn_create_graphics_pipelines)(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,)
    }

    #[inline]
    pub unsafe fn merge_pipeline_caches(&self,
        device: VkDevice,
        dst_cache: VkPipelineCache,
        src_cache_count: u32,
        p_src_caches: *const VkPipelineCache) -> VkResult {
        (self.pfn_merge_pipeline_caches)(
            device,
            dst_cache,
            src_cache_count,
            p_src_caches,)
    }

    #[inline]
    pub unsafe fn get_pipeline_cache_data(&self,
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        p_data_size: *mut usize,
        p_data: *mut core::ffi::c_void) -> VkResult {
        (self.pfn_get_pipeline_cache_data)(
            device,
            pipeline_cache,
            p_data_size,
            p_data,)
    }

    #[inline]
    pub unsafe fn destroy_pipeline_cache(&self,
        device: VkDevice,
        pipeline_cache: VkPipelineCache,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_pipeline_cache)(
            device,
            pipeline_cache,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_pipeline_cache(&self,
        device: VkDevice,
        p_create_info: *const VkPipelineCacheCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_pipeline_cache: *mut VkPipelineCache) -> VkResult {
        (self.pfn_create_pipeline_cache)(
            device,
            p_create_info,
            p_allocator,
            p_pipeline_cache,)
    }

    #[inline]
    pub unsafe fn destroy_shader_module(&self,
        device: VkDevice,
        shader_module: VkShaderModule,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_shader_module)(
            device,
            shader_module,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_shader_module(&self,
        device: VkDevice,
        p_create_info: *const VkShaderModuleCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_shader_module: *mut VkShaderModule) -> VkResult {
        (self.pfn_create_shader_module)(
            device,
            p_create_info,
            p_allocator,
            p_shader_module,)
    }

    #[inline]
    pub unsafe fn destroy_image_view(&self,
        device: VkDevice,
        image_view: VkImageView,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_image_view)(
            device,
            image_view,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_image_view(&self,
        device: VkDevice,
        p_create_info: *const VkImageViewCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_view: *mut VkImageView) -> VkResult {
        (self.pfn_create_image_view)(
            device,
            p_create_info,
            p_allocator,
            p_view,)
    }

    #[inline]
    pub unsafe fn get_image_subresource_layout(&self,
        device: VkDevice,
        image: VkImage,
        p_subresource: *const VkImageSubresource,
        p_layout: *mut VkSubresourceLayout) {
        (self.pfn_get_image_subresource_layout)(
            device,
            image,
            p_subresource,
            p_layout,)
    }

    #[inline]
    pub unsafe fn destroy_image(&self,
        device: VkDevice,
        image: VkImage,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_image)(
            device,
            image,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_image(&self,
        device: VkDevice,
        p_create_info: *const VkImageCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_image: *mut VkImage) -> VkResult {
        (self.pfn_create_image)(
            device,
            p_create_info,
            p_allocator,
            p_image,)
    }

    #[inline]
    pub unsafe fn destroy_buffer_view(&self,
        device: VkDevice,
        buffer_view: VkBufferView,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_buffer_view)(
            device,
            buffer_view,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_buffer_view(&self,
        device: VkDevice,
        p_create_info: *const VkBufferViewCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_view: *mut VkBufferView) -> VkResult {
        (self.pfn_create_buffer_view)(
            device,
            p_create_info,
            p_allocator,
            p_view,)
    }

    #[inline]
    pub unsafe fn destroy_buffer(&self,
        device: VkDevice,
        buffer: VkBuffer,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_buffer)(
            device,
            buffer,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_buffer(&self,
        device: VkDevice,
        p_create_info: *const VkBufferCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_buffer: *mut VkBuffer) -> VkResult {
        (self.pfn_create_buffer)(
            device,
            p_create_info,
            p_allocator,
            p_buffer,)
    }

    #[inline]
    pub unsafe fn get_query_pool_results(&self,
        device: VkDevice,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut core::ffi::c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags) -> VkResult {
        (self.pfn_get_query_pool_results)(
            device,
            query_pool,
            first_query,
            query_count,
            data_size,
            p_data,
            stride,
            flags,)
    }

    #[inline]
    pub unsafe fn destroy_query_pool(&self,
        device: VkDevice,
        query_pool: VkQueryPool,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_query_pool)(
            device,
            query_pool,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_query_pool(&self,
        device: VkDevice,
        p_create_info: *const VkQueryPoolCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_query_pool: *mut VkQueryPool) -> VkResult {
        (self.pfn_create_query_pool)(
            device,
            p_create_info,
            p_allocator,
            p_query_pool,)
    }

    #[inline]
    pub unsafe fn reset_event(&self,
        device: VkDevice,
        event: VkEvent) -> VkResult {
        (self.pfn_reset_event)(
            device,
            event,)
    }

    #[inline]
    pub unsafe fn set_event(&self,
        device: VkDevice,
        event: VkEvent) -> VkResult {
        (self.pfn_set_event)(
            device,
            event,)
    }

    #[inline]
    pub unsafe fn get_event_status(&self,
        device: VkDevice,
        event: VkEvent) -> VkResult {
        (self.pfn_get_event_status)(
            device,
            event,)
    }

    #[inline]
    pub unsafe fn destroy_event(&self,
        device: VkDevice,
        event: VkEvent,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_event)(
            device,
            event,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_event(&self,
        device: VkDevice,
        p_create_info: *const VkEventCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_event: *mut VkEvent) -> VkResult {
        (self.pfn_create_event)(
            device,
            p_create_info,
            p_allocator,
            p_event,)
    }

    #[inline]
    pub unsafe fn destroy_semaphore(&self,
        device: VkDevice,
        semaphore: VkSemaphore,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_semaphore)(
            device,
            semaphore,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_semaphore(&self,
        device: VkDevice,
        p_create_info: *const VkSemaphoreCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_semaphore: *mut VkSemaphore) -> VkResult {
        (self.pfn_create_semaphore)(
            device,
            p_create_info,
            p_allocator,
            p_semaphore,)
    }

    #[inline]
    pub unsafe fn wait_for_fences(&self,
        device: VkDevice,
        fence_count: u32,
        p_fences: *const VkFence,
        wait_all: VkBool32,
        timeout: u64) -> VkResult {
        (self.pfn_wait_for_fences)(
            device,
            fence_count,
            p_fences,
            wait_all,
            timeout,)
    }

    #[inline]
    pub unsafe fn get_fence_status(&self,
        device: VkDevice,
        fence: VkFence) -> VkResult {
        (self.pfn_get_fence_status)(
            device,
            fence,)
    }

    #[inline]
    pub unsafe fn reset_fences(&self,
        device: VkDevice,
        fence_count: u32,
        p_fences: *const VkFence) -> VkResult {
        (self.pfn_reset_fences)(
            device,
            fence_count,
            p_fences,)
    }

    #[inline]
    pub unsafe fn destroy_fence(&self,
        device: VkDevice,
        fence: VkFence,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_fence)(
            device,
            fence,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn create_fence(&self,
        device: VkDevice,
        p_create_info: *const VkFenceCreateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_fence: *mut VkFence) -> VkResult {
        (self.pfn_create_fence)(
            device,
            p_create_info,
            p_allocator,
            p_fence,)
    }

    #[inline]
    pub unsafe fn queue_bind_sparse(&self,
        queue: VkQueue,
        bind_info_count: u32,
        p_bind_info: *const VkBindSparseInfo,
        fence: VkFence) -> VkResult {
        (self.pfn_queue_bind_sparse)(
            queue,
            bind_info_count,
            p_bind_info,
            fence,)
    }

    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements(&self,
        device: VkDevice,
        image: VkImage,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut VkSparseImageMemoryRequirements) {
        (self.pfn_get_image_sparse_memory_requirements)(
            device,
            image,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,)
    }

    #[inline]
    pub unsafe fn get_image_memory_requirements(&self,
        device: VkDevice,
        image: VkImage,
        p_memory_requirements: *mut VkMemoryRequirements) {
        (self.pfn_get_image_memory_requirements)(
            device,
            image,
            p_memory_requirements,)
    }

    #[inline]
    pub unsafe fn get_buffer_memory_requirements(&self,
        device: VkDevice,
        buffer: VkBuffer,
        p_memory_requirements: *mut VkMemoryRequirements) {
        (self.pfn_get_buffer_memory_requirements)(
            device,
            buffer,
            p_memory_requirements,)
    }

    #[inline]
    pub unsafe fn bind_image_memory(&self,
        device: VkDevice,
        image: VkImage,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize) -> VkResult {
        (self.pfn_bind_image_memory)(
            device,
            image,
            memory,
            memory_offset,)
    }

    #[inline]
    pub unsafe fn bind_buffer_memory(&self,
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize) -> VkResult {
        (self.pfn_bind_buffer_memory)(
            device,
            buffer,
            memory,
            memory_offset,)
    }

    #[inline]
    pub unsafe fn get_device_memory_commitment(&self,
        device: VkDevice,
        memory: VkDeviceMemory,
        p_committed_memory_in_bytes: *mut VkDeviceSize) {
        (self.pfn_get_device_memory_commitment)(
            device,
            memory,
            p_committed_memory_in_bytes,)
    }

    #[inline]
    pub unsafe fn invalidate_mapped_memory_ranges(&self,
        device: VkDevice,
        memory_range_count: u32,
        p_memory_ranges: *const VkMappedMemoryRange) -> VkResult {
        (self.pfn_invalidate_mapped_memory_ranges)(
            device,
            memory_range_count,
            p_memory_ranges,)
    }

    #[inline]
    pub unsafe fn flush_mapped_memory_ranges(&self,
        device: VkDevice,
        memory_range_count: u32,
        p_memory_ranges: *const VkMappedMemoryRange) -> VkResult {
        (self.pfn_flush_mapped_memory_ranges)(
            device,
            memory_range_count,
            p_memory_ranges,)
    }

    #[inline]
    pub unsafe fn unmap_memory(&self,
        device: VkDevice,
        memory: VkDeviceMemory) {
        (self.pfn_unmap_memory)(
            device,
            memory,)
    }

    #[inline]
    pub unsafe fn map_memory(&self,
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        pp_data: *mut *mut core::ffi::c_void) -> VkResult {
        (self.pfn_map_memory)(
            device,
            memory,
            offset,
            size,
            flags,
            pp_data,)
    }

    #[inline]
    pub unsafe fn free_memory(&self,
        device: VkDevice,
        memory: VkDeviceMemory,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_free_memory)(
            device,
            memory,
            p_allocator,)
    }

    #[inline]
    pub unsafe fn allocate_memory(&self,
        device: VkDevice,
        p_allocate_info: *const VkMemoryAllocateInfo,
        p_allocator: *const VkAllocationCallbacks,
        p_memory: *mut VkDeviceMemory) -> VkResult {
        (self.pfn_allocate_memory)(
            device,
            p_allocate_info,
            p_allocator,
            p_memory,)
    }

    #[inline]
    pub unsafe fn device_wait_idle(&self,
        device: VkDevice) -> VkResult {
        (self.pfn_device_wait_idle)(
            device,)
    }

    #[inline]
    pub unsafe fn queue_wait_idle(&self,
        queue: VkQueue) -> VkResult {
        (self.pfn_queue_wait_idle)(
            queue,)
    }

    #[inline]
    pub unsafe fn queue_submit(&self,
        queue: VkQueue,
        submit_count: u32,
        p_submits: *const VkSubmitInfo,
        fence: VkFence) -> VkResult {
        (self.pfn_queue_submit)(
            queue,
            submit_count,
            p_submits,
            fence,)
    }

    #[inline]
    pub unsafe fn get_device_queue(&self,
        device: VkDevice,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut VkQueue) {
        (self.pfn_get_device_queue)(
            device,
            queue_family_index,
            queue_index,
            p_queue,)
    }

    #[inline]
    pub unsafe fn destroy_device(&self,
        device: VkDevice,
        p_allocator: *const VkAllocationCallbacks) {
        (self.pfn_destroy_device)(
            device,
            p_allocator,)
    }
}

