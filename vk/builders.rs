use crate::utils::*;
use crate::types::*;

pub struct VkAllocationCallbacksBuilder<'a> {
    s: VkAllocationCallbacks,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkAllocationCallbacksBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkAllocationCallbacks::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn p_user_data(mut self, value: Option<&'a mut core::ffi::c_void>) -> VkAllocationCallbacksBuilder<'a> {
        self.s.p_user_data = match value {
            Some(r) => r,
            None => core::ptr::null_mut(),
        };
        self
    }

    pub fn pfn_allocation(mut self, value: PfnVkAllocationFunction) -> VkAllocationCallbacksBuilder<'a> {
        self.s.pfn_allocation = value;
        self
    }

    pub fn pfn_reallocation(mut self, value: PfnVkReallocationFunction) -> VkAllocationCallbacksBuilder<'a> {
        self.s.pfn_reallocation = value;
        self
    }

    pub fn pfn_free(mut self, value: PfnVkFreeFunction) -> VkAllocationCallbacksBuilder<'a> {
        self.s.pfn_free = value;
        self
    }

    pub fn pfn_internal_allocation(mut self, value: PfnVkInternalAllocationNotification) -> VkAllocationCallbacksBuilder<'a> {
        self.s.pfn_internal_allocation = value;
        self
    }

    pub fn pfn_internal_free(mut self, value: PfnVkInternalFreeNotification) -> VkAllocationCallbacksBuilder<'a> {
        self.s.pfn_internal_free = value;
        self
    }
}

impl<'a> core::ops::Deref for VkAllocationCallbacksBuilder<'a> {
    type Target = VkAllocationCallbacks;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkAllocationCallbacksBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkWin32SurfaceCreateInfoKHRBuilder<'a> {
    s: VkWin32SurfaceCreateInfoKHR,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkWin32SurfaceCreateInfoKHRBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkWin32SurfaceCreateInfoKHR::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkWin32SurfaceCreateInfoKHRBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsWin32SurfaceCreateInfoKHR>(mut self, next: &'a mut T) -> VkWin32SurfaceCreateInfoKHRBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkWin32SurfaceCreateFlagsKHR) -> VkWin32SurfaceCreateInfoKHRBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn hinstance(mut self, value: HINSTANCE) -> VkWin32SurfaceCreateInfoKHRBuilder<'a> {
        self.s.hinstance = value;
        self
    }

    pub fn hwnd(mut self, value: HWND) -> VkWin32SurfaceCreateInfoKHRBuilder<'a> {
        self.s.hwnd = value;
        self
    }
}

impl<'a> core::ops::Deref for VkWin32SurfaceCreateInfoKHRBuilder<'a> {
    type Target = VkWin32SurfaceCreateInfoKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkWin32SurfaceCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSurfaceFormatKHRBuilder<'a> {
    s: VkSurfaceFormatKHR,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSurfaceFormatKHRBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSurfaceFormatKHR::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn format(mut self, value: VkFormat) -> VkSurfaceFormatKHRBuilder<'a> {
        self.s.format = value;
        self
    }

    pub fn color_space(mut self, value: VkColorSpaceKHR) -> VkSurfaceFormatKHRBuilder<'a> {
        self.s.color_space = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSurfaceFormatKHRBuilder<'a> {
    type Target = VkSurfaceFormatKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSurfaceFormatKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSurfaceCapabilitiesKHRBuilder<'a> {
    s: VkSurfaceCapabilitiesKHR,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSurfaceCapabilitiesKHRBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSurfaceCapabilitiesKHR::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn min_image_count(mut self, value: u32) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.min_image_count = value;
        self
    }

    pub fn max_image_count(mut self, value: u32) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.max_image_count = value;
        self
    }

    pub fn current_extent(mut self, value: VkExtent2D) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.current_extent = value;
        self
    }

    pub fn min_image_extent(mut self, value: VkExtent2D) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.min_image_extent = value;
        self
    }

    pub fn max_image_extent(mut self, value: VkExtent2D) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.max_image_extent = value;
        self
    }

    pub fn max_image_array_layers(mut self, value: u32) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.max_image_array_layers = value;
        self
    }

    pub fn supported_transforms(mut self, value: VkSurfaceTransformFlagsKHR) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.supported_transforms = value;
        self
    }

    pub fn current_transform(mut self, value: VkSurfaceTransformFlagBitsKHR) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.current_transform = value;
        self
    }

    pub fn supported_composite_alpha(mut self, value: VkCompositeAlphaFlagsKHR) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.supported_composite_alpha = value;
        self
    }

    pub fn supported_usage_flags(mut self, value: VkImageUsageFlags) -> VkSurfaceCapabilitiesKHRBuilder<'a> {
        self.s.supported_usage_flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSurfaceCapabilitiesKHRBuilder<'a> {
    type Target = VkSurfaceCapabilitiesKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSurfaceCapabilitiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkExtent2DBuilder<'a> {
    s: VkExtent2D,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkExtent2DBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkExtent2D::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn width(mut self, value: u32) -> VkExtent2DBuilder<'a> {
        self.s.width = value;
        self
    }

    pub fn height(mut self, value: u32) -> VkExtent2DBuilder<'a> {
        self.s.height = value;
        self
    }
}

impl<'a> core::ops::Deref for VkExtent2DBuilder<'a> {
    type Target = VkExtent2D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkExtent2DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPresentInfoKHRBuilder<'a> {
    s: VkPresentInfoKHR,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPresentInfoKHRBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPresentInfoKHR::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPresentInfoKHRBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPresentInfoKHR>(mut self, next: &'a mut T) -> VkPresentInfoKHRBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn wait_semaphore_count(mut self, value: u32) -> VkPresentInfoKHRBuilder<'a> {
        self.s.wait_semaphore_count = value;
        self
    }

    pub fn p_wait_semaphores(mut self, values: &'a [VkSemaphore]) -> VkPresentInfoKHRBuilder<'a> {
        self.s.wait_semaphore_count = values.len() as _;
        self.s.p_wait_semaphores = values.as_ptr();
        self
    }

    pub fn swapchain_count(mut self, value: u32) -> VkPresentInfoKHRBuilder<'a> {
        self.s.swapchain_count = value;
        self
    }

    pub fn p_swapchains(mut self, values: &'a [VkSwapchainKHR]) -> VkPresentInfoKHRBuilder<'a> {
        self.s.swapchain_count = values.len() as _;
        self.s.p_swapchains = values.as_ptr();
        self
    }

    pub fn p_image_indices(mut self, values: &'a [u32]) -> VkPresentInfoKHRBuilder<'a> {
        self.s.swapchain_count = values.len() as _;
        self.s.p_image_indices = values.as_ptr();
        self
    }

    pub fn p_results(mut self, values: &'a mut[VkResult]) -> VkPresentInfoKHRBuilder<'a> {
        self.s.swapchain_count = values.len() as _;
        self.s.p_results = values.as_mut_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkPresentInfoKHRBuilder<'a> {
    type Target = VkPresentInfoKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPresentInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSwapchainCreateInfoKHRBuilder<'a> {
    s: VkSwapchainCreateInfoKHR,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSwapchainCreateInfoKHRBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSwapchainCreateInfoKHR::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsSwapchainCreateInfoKHR>(mut self, next: &'a mut T) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkSwapchainCreateFlagsKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn surface(mut self, value: VkSurfaceKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.surface = value;
        self
    }

    pub fn min_image_count(mut self, value: u32) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.min_image_count = value;
        self
    }

    pub fn image_format(mut self, value: VkFormat) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.image_format = value;
        self
    }

    pub fn image_color_space(mut self, value: VkColorSpaceKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.image_color_space = value;
        self
    }

    pub fn image_extent(mut self, value: VkExtent2D) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.image_extent = value;
        self
    }

    pub fn image_array_layers(mut self, value: u32) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.image_array_layers = value;
        self
    }

    pub fn image_usage(mut self, value: VkImageUsageFlags) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.image_usage = value;
        self
    }

    pub fn image_sharing_mode(mut self, value: VkSharingMode) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.image_sharing_mode = value;
        self
    }

    pub fn queue_family_index_count(mut self, value: u32) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.queue_family_index_count = value;
        self
    }

    pub fn p_queue_family_indices(mut self, values: &'a [u32]) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.queue_family_index_count = values.len() as _;
        self.s.p_queue_family_indices = values.as_ptr();
        self
    }

    pub fn pre_transform(mut self, value: VkSurfaceTransformFlagBitsKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.pre_transform = value;
        self
    }

    pub fn composite_alpha(mut self, value: VkCompositeAlphaFlagBitsKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.composite_alpha = value;
        self
    }

    pub fn present_mode(mut self, value: VkPresentModeKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.present_mode = value;
        self
    }

    pub fn clipped(mut self, value: bool) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.clipped = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn old_swapchain(mut self, value: VkSwapchainKHR) -> VkSwapchainCreateInfoKHRBuilder<'a> {
        self.s.old_swapchain = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSwapchainCreateInfoKHRBuilder<'a> {
    type Target = VkSwapchainCreateInfoKHR;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSwapchainCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    s: VkDebugUtilsMessengerCallbackDataEXT,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsMessengerCallbackDataEXT::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDebugUtilsMessengerCallbackDataEXT>(mut self, next: &'a mut T) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkDebugUtilsMessengerCallbackDataFlagsEXT) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn p_message_id_name(mut self, values: &'a [u8]) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        
        self.s.p_message_id_name = values.as_ptr();
        self
    }

    pub fn message_id_number(mut self, value: i32) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.message_id_number = value;
        self
    }

    pub fn p_message(mut self, values: &'a [u8]) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        
        self.s.p_message = values.as_ptr();
        self
    }

    pub fn queue_label_count(mut self, value: u32) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.queue_label_count = value;
        self
    }

    pub fn p_queue_labels(mut self, values: &'a [VkDebugUtilsLabelEXT]) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.queue_label_count = values.len() as _;
        self.s.p_queue_labels = values.as_ptr();
        self
    }

    pub fn cmd_buf_label_count(mut self, value: u32) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.cmd_buf_label_count = value;
        self
    }

    pub fn p_cmd_buf_labels(mut self, values: &'a [VkDebugUtilsLabelEXT]) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.cmd_buf_label_count = values.len() as _;
        self.s.p_cmd_buf_labels = values.as_ptr();
        self
    }

    pub fn object_count(mut self, value: u32) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.object_count = value;
        self
    }

    pub fn p_objects(mut self, values: &'a [VkDebugUtilsObjectNameInfoEXT]) -> VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
        self.s.object_count = values.len() as _;
        self.s.p_objects = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    type Target = VkDebugUtilsMessengerCallbackDataEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
    s: VkDebugUtilsObjectNameInfoEXT,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsObjectNameInfoEXT::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDebugUtilsObjectNameInfoEXT>(mut self, next: &'a mut T) -> VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn object_type(mut self, value: VkObjectType) -> VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
        self.s.object_type = value;
        self
    }

    pub fn object_handle(mut self, value: u64) -> VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
        self.s.object_handle = value;
        self
    }

    pub fn p_object_name(mut self, values: &'a [u8]) -> VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
        
        self.s.p_object_name = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
    type Target = VkDebugUtilsObjectNameInfoEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDebugUtilsObjectNameInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsLabelEXTBuilder<'a> {
    s: VkDebugUtilsLabelEXT,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDebugUtilsLabelEXTBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsLabelEXT::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDebugUtilsLabelEXTBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDebugUtilsLabelEXT>(mut self, next: &'a mut T) -> VkDebugUtilsLabelEXTBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn p_label_name(mut self, values: &'a [u8]) -> VkDebugUtilsLabelEXTBuilder<'a> {
        
        self.s.p_label_name = values.as_ptr();
        self
    }

    pub fn color(mut self, value: [f32; 4]) -> VkDebugUtilsLabelEXTBuilder<'a> {
        self.s.color = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDebugUtilsLabelEXTBuilder<'a> {
    type Target = VkDebugUtilsLabelEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDebugUtilsLabelEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    s: VkDebugUtilsMessengerCreateInfoEXT,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsMessengerCreateInfoEXT::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDebugUtilsMessengerCreateInfoEXT>(mut self, next: &'a mut T) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkDebugUtilsMessengerCreateFlagsEXT) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn message_severity(mut self, value: VkDebugUtilsMessageSeverityFlagsEXT) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        self.s.message_severity = value;
        self
    }

    pub fn message_type(mut self, value: VkDebugUtilsMessageTypeFlagsEXT) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        self.s.message_type = value;
        self
    }

    pub fn pfn_user_callback(mut self, value: PfnVkDebugUtilsMessengerCallbackEXT) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        self.s.pfn_user_callback = value;
        self
    }

    pub fn p_user_data(mut self, value: Option<&'a mut core::ffi::c_void>) -> VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        self.s.p_user_data = match value {
            Some(r) => r,
            None => core::ptr::null_mut(),
        };
        self
    }
}

impl<'a> core::ops::Deref for VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    type Target = VkDebugUtilsMessengerCreateInfoEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
    s: VkDebugUtilsObjectTagInfoEXT,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDebugUtilsObjectTagInfoEXT::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDebugUtilsObjectTagInfoEXT>(mut self, next: &'a mut T) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn object_type(mut self, value: VkObjectType) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        self.s.object_type = value;
        self
    }

    pub fn object_handle(mut self, value: u64) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        self.s.object_handle = value;
        self
    }

    pub fn tag_name(mut self, value: u64) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        self.s.tag_name = value;
        self
    }

    pub fn tag_size(mut self, value: usize) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        self.s.tag_size = value;
        self
    }

    pub fn p_tag(mut self, values: &'a [core::ffi::c_void]) -> VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
        self.s.tag_size = values.len() as _;
        self.s.p_tag = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
    type Target = VkDebugUtilsObjectTagInfoEXT;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDebugUtilsObjectTagInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBaseInStructureBuilder<'a> {
    s: VkBaseInStructure,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBaseInStructureBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBaseInStructure::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkBaseInStructureBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsBaseInStructure>(mut self, next: &'a mut T) -> VkBaseInStructureBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }
}

impl<'a> core::ops::Deref for VkBaseInStructureBuilder<'a> {
    type Target = VkBaseInStructure;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBaseInStructureBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBaseOutStructureBuilder<'a> {
    s: VkBaseOutStructure,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBaseOutStructureBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBaseOutStructure::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkBaseOutStructureBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsBaseOutStructure>(mut self, next: &'a mut T) -> VkBaseOutStructureBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }
}

impl<'a> core::ops::Deref for VkBaseOutStructureBuilder<'a> {
    type Target = VkBaseOutStructure;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBaseOutStructureBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryBarrierBuilder<'a> {
    s: VkMemoryBarrier,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkMemoryBarrierBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkMemoryBarrier::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkMemoryBarrierBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsMemoryBarrier>(mut self, next: &'a mut T) -> VkMemoryBarrierBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn src_access_mask(mut self, value: VkAccessFlags) -> VkMemoryBarrierBuilder<'a> {
        self.s.src_access_mask = value;
        self
    }

    pub fn dst_access_mask(mut self, value: VkAccessFlags) -> VkMemoryBarrierBuilder<'a> {
        self.s.dst_access_mask = value;
        self
    }
}

impl<'a> core::ops::Deref for VkMemoryBarrierBuilder<'a> {
    type Target = VkMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkMemoryBarrierBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageMemoryBarrierBuilder<'a> {
    s: VkImageMemoryBarrier,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageMemoryBarrierBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageMemoryBarrier::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsImageMemoryBarrier>(mut self, next: &'a mut T) -> VkImageMemoryBarrierBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn src_access_mask(mut self, value: VkAccessFlags) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.src_access_mask = value;
        self
    }

    pub fn dst_access_mask(mut self, value: VkAccessFlags) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.dst_access_mask = value;
        self
    }

    pub fn old_layout(mut self, value: VkImageLayout) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.old_layout = value;
        self
    }

    pub fn new_layout(mut self, value: VkImageLayout) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.new_layout = value;
        self
    }

    pub fn src_queue_family_index(mut self, value: u32) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.src_queue_family_index = value;
        self
    }

    pub fn dst_queue_family_index(mut self, value: u32) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.dst_queue_family_index = value;
        self
    }

    pub fn image(mut self, value: VkImage) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.image = value;
        self
    }

    pub fn subresource_range(mut self, value: VkImageSubresourceRange) -> VkImageMemoryBarrierBuilder<'a> {
        self.s.subresource_range = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageMemoryBarrierBuilder<'a> {
    type Target = VkImageMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageMemoryBarrierBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageSubresourceRangeBuilder<'a> {
    s: VkImageSubresourceRange,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageSubresourceRangeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageSubresourceRange::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn aspect_mask(mut self, value: VkImageAspectFlags) -> VkImageSubresourceRangeBuilder<'a> {
        self.s.aspect_mask = value;
        self
    }

    pub fn base_mip_level(mut self, value: u32) -> VkImageSubresourceRangeBuilder<'a> {
        self.s.base_mip_level = value;
        self
    }

    pub fn level_count(mut self, value: u32) -> VkImageSubresourceRangeBuilder<'a> {
        self.s.level_count = value;
        self
    }

    pub fn base_array_layer(mut self, value: u32) -> VkImageSubresourceRangeBuilder<'a> {
        self.s.base_array_layer = value;
        self
    }

    pub fn layer_count(mut self, value: u32) -> VkImageSubresourceRangeBuilder<'a> {
        self.s.layer_count = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageSubresourceRangeBuilder<'a> {
    type Target = VkImageSubresourceRange;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageSubresourceRangeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDrawIndirectCommandBuilder<'a> {
    s: VkDrawIndirectCommand,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDrawIndirectCommandBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDrawIndirectCommand::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn vertex_count(mut self, value: u32) -> VkDrawIndirectCommandBuilder<'a> {
        self.s.vertex_count = value;
        self
    }

    pub fn instance_count(mut self, value: u32) -> VkDrawIndirectCommandBuilder<'a> {
        self.s.instance_count = value;
        self
    }

    pub fn first_vertex(mut self, value: u32) -> VkDrawIndirectCommandBuilder<'a> {
        self.s.first_vertex = value;
        self
    }

    pub fn first_instance(mut self, value: u32) -> VkDrawIndirectCommandBuilder<'a> {
        self.s.first_instance = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDrawIndirectCommandBuilder<'a> {
    type Target = VkDrawIndirectCommand;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDrawIndirectCommandBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDrawIndexedIndirectCommandBuilder<'a> {
    s: VkDrawIndexedIndirectCommand,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDrawIndexedIndirectCommandBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDrawIndexedIndirectCommand::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn index_count(mut self, value: u32) -> VkDrawIndexedIndirectCommandBuilder<'a> {
        self.s.index_count = value;
        self
    }

    pub fn instance_count(mut self, value: u32) -> VkDrawIndexedIndirectCommandBuilder<'a> {
        self.s.instance_count = value;
        self
    }

    pub fn first_index(mut self, value: u32) -> VkDrawIndexedIndirectCommandBuilder<'a> {
        self.s.first_index = value;
        self
    }

    pub fn vertex_offset(mut self, value: i32) -> VkDrawIndexedIndirectCommandBuilder<'a> {
        self.s.vertex_offset = value;
        self
    }

    pub fn first_instance(mut self, value: u32) -> VkDrawIndexedIndirectCommandBuilder<'a> {
        self.s.first_instance = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDrawIndexedIndirectCommandBuilder<'a> {
    type Target = VkDrawIndexedIndirectCommand;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDrawIndexedIndirectCommandBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDispatchIndirectCommandBuilder<'a> {
    s: VkDispatchIndirectCommand,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDispatchIndirectCommandBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDispatchIndirectCommand::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn x(mut self, value: u32) -> VkDispatchIndirectCommandBuilder<'a> {
        self.s.x = value;
        self
    }

    pub fn y(mut self, value: u32) -> VkDispatchIndirectCommandBuilder<'a> {
        self.s.y = value;
        self
    }

    pub fn z(mut self, value: u32) -> VkDispatchIndirectCommandBuilder<'a> {
        self.s.z = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDispatchIndirectCommandBuilder<'a> {
    type Target = VkDispatchIndirectCommand;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDispatchIndirectCommandBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferMemoryBarrierBuilder<'a> {
    s: VkBufferMemoryBarrier,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBufferMemoryBarrierBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBufferMemoryBarrier::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsBufferMemoryBarrier>(mut self, next: &'a mut T) -> VkBufferMemoryBarrierBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn src_access_mask(mut self, value: VkAccessFlags) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.src_access_mask = value;
        self
    }

    pub fn dst_access_mask(mut self, value: VkAccessFlags) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.dst_access_mask = value;
        self
    }

    pub fn src_queue_family_index(mut self, value: u32) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.src_queue_family_index = value;
        self
    }

    pub fn dst_queue_family_index(mut self, value: u32) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.dst_queue_family_index = value;
        self
    }

    pub fn buffer(mut self, value: VkBuffer) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.buffer = value;
        self
    }

    pub fn offset(mut self, value: VkDeviceSize) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkBufferMemoryBarrierBuilder<'a> {
        self.s.size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkBufferMemoryBarrierBuilder<'a> {
    type Target = VkBufferMemoryBarrier;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBufferMemoryBarrierBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkRenderPassBeginInfoBuilder<'a> {
    s: VkRenderPassBeginInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkRenderPassBeginInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkRenderPassBeginInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkRenderPassBeginInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsRenderPassBeginInfo>(mut self, next: &'a mut T) -> VkRenderPassBeginInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn render_pass(mut self, value: VkRenderPass) -> VkRenderPassBeginInfoBuilder<'a> {
        self.s.render_pass = value;
        self
    }

    pub fn framebuffer(mut self, value: VkFramebuffer) -> VkRenderPassBeginInfoBuilder<'a> {
        self.s.framebuffer = value;
        self
    }

    pub fn render_area(mut self, value: VkRect2D) -> VkRenderPassBeginInfoBuilder<'a> {
        self.s.render_area = value;
        self
    }

    pub fn clear_value_count(mut self, value: u32) -> VkRenderPassBeginInfoBuilder<'a> {
        self.s.clear_value_count = value;
        self
    }

    pub fn p_clear_values(mut self, values: &'a [VkClearValue]) -> VkRenderPassBeginInfoBuilder<'a> {
        self.s.clear_value_count = values.len() as _;
        self.s.p_clear_values = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkRenderPassBeginInfoBuilder<'a> {
    type Target = VkRenderPassBeginInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkRenderPassBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkClearDepthStencilValueBuilder<'a> {
    s: VkClearDepthStencilValue,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkClearDepthStencilValueBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkClearDepthStencilValue::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn depth(mut self, value: f32) -> VkClearDepthStencilValueBuilder<'a> {
        self.s.depth = value;
        self
    }

    pub fn stencil(mut self, value: u32) -> VkClearDepthStencilValueBuilder<'a> {
        self.s.stencil = value;
        self
    }
}

impl<'a> core::ops::Deref for VkClearDepthStencilValueBuilder<'a> {
    type Target = VkClearDepthStencilValue;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkClearDepthStencilValueBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkRect2DBuilder<'a> {
    s: VkRect2D,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkRect2DBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkRect2D::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn offset(mut self, value: VkOffset2D) -> VkRect2DBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn extent(mut self, value: VkExtent2D) -> VkRect2DBuilder<'a> {
        self.s.extent = value;
        self
    }
}

impl<'a> core::ops::Deref for VkRect2DBuilder<'a> {
    type Target = VkRect2D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkRect2DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkOffset2DBuilder<'a> {
    s: VkOffset2D,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkOffset2DBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkOffset2D::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn x(mut self, value: i32) -> VkOffset2DBuilder<'a> {
        self.s.x = value;
        self
    }

    pub fn y(mut self, value: i32) -> VkOffset2DBuilder<'a> {
        self.s.y = value;
        self
    }
}

impl<'a> core::ops::Deref for VkOffset2DBuilder<'a> {
    type Target = VkOffset2D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkOffset2DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageResolveBuilder<'a> {
    s: VkImageResolve,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageResolveBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageResolve::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn src_subresource(mut self, value: VkImageSubresourceLayers) -> VkImageResolveBuilder<'a> {
        self.s.src_subresource = value;
        self
    }

    pub fn src_offset(mut self, value: VkOffset3D) -> VkImageResolveBuilder<'a> {
        self.s.src_offset = value;
        self
    }

    pub fn dst_subresource(mut self, value: VkImageSubresourceLayers) -> VkImageResolveBuilder<'a> {
        self.s.dst_subresource = value;
        self
    }

    pub fn dst_offset(mut self, value: VkOffset3D) -> VkImageResolveBuilder<'a> {
        self.s.dst_offset = value;
        self
    }

    pub fn extent(mut self, value: VkExtent3D) -> VkImageResolveBuilder<'a> {
        self.s.extent = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageResolveBuilder<'a> {
    type Target = VkImageResolve;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageResolveBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkExtent3DBuilder<'a> {
    s: VkExtent3D,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkExtent3DBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkExtent3D::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn width(mut self, value: u32) -> VkExtent3DBuilder<'a> {
        self.s.width = value;
        self
    }

    pub fn height(mut self, value: u32) -> VkExtent3DBuilder<'a> {
        self.s.height = value;
        self
    }

    pub fn depth(mut self, value: u32) -> VkExtent3DBuilder<'a> {
        self.s.depth = value;
        self
    }
}

impl<'a> core::ops::Deref for VkExtent3DBuilder<'a> {
    type Target = VkExtent3D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkExtent3DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkOffset3DBuilder<'a> {
    s: VkOffset3D,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkOffset3DBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkOffset3D::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn x(mut self, value: i32) -> VkOffset3DBuilder<'a> {
        self.s.x = value;
        self
    }

    pub fn y(mut self, value: i32) -> VkOffset3DBuilder<'a> {
        self.s.y = value;
        self
    }

    pub fn z(mut self, value: i32) -> VkOffset3DBuilder<'a> {
        self.s.z = value;
        self
    }
}

impl<'a> core::ops::Deref for VkOffset3DBuilder<'a> {
    type Target = VkOffset3D;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkOffset3DBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageSubresourceLayersBuilder<'a> {
    s: VkImageSubresourceLayers,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageSubresourceLayersBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageSubresourceLayers::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn aspect_mask(mut self, value: VkImageAspectFlags) -> VkImageSubresourceLayersBuilder<'a> {
        self.s.aspect_mask = value;
        self
    }

    pub fn mip_level(mut self, value: u32) -> VkImageSubresourceLayersBuilder<'a> {
        self.s.mip_level = value;
        self
    }

    pub fn base_array_layer(mut self, value: u32) -> VkImageSubresourceLayersBuilder<'a> {
        self.s.base_array_layer = value;
        self
    }

    pub fn layer_count(mut self, value: u32) -> VkImageSubresourceLayersBuilder<'a> {
        self.s.layer_count = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageSubresourceLayersBuilder<'a> {
    type Target = VkImageSubresourceLayers;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageSubresourceLayersBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkClearRectBuilder<'a> {
    s: VkClearRect,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkClearRectBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkClearRect::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn rect(mut self, value: VkRect2D) -> VkClearRectBuilder<'a> {
        self.s.rect = value;
        self
    }

    pub fn base_array_layer(mut self, value: u32) -> VkClearRectBuilder<'a> {
        self.s.base_array_layer = value;
        self
    }

    pub fn layer_count(mut self, value: u32) -> VkClearRectBuilder<'a> {
        self.s.layer_count = value;
        self
    }
}

impl<'a> core::ops::Deref for VkClearRectBuilder<'a> {
    type Target = VkClearRect;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkClearRectBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkClearAttachmentBuilder<'a> {
    s: VkClearAttachment,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkClearAttachmentBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkClearAttachment::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn aspect_mask(mut self, value: VkImageAspectFlags) -> VkClearAttachmentBuilder<'a> {
        self.s.aspect_mask = value;
        self
    }

    pub fn color_attachment(mut self, value: u32) -> VkClearAttachmentBuilder<'a> {
        self.s.color_attachment = value;
        self
    }

    pub fn clear_value(mut self, value: VkClearValue) -> VkClearAttachmentBuilder<'a> {
        self.s.clear_value = value;
        self
    }
}

impl<'a> core::ops::Deref for VkClearAttachmentBuilder<'a> {
    type Target = VkClearAttachment;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkClearAttachmentBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferImageCopyBuilder<'a> {
    s: VkBufferImageCopy,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBufferImageCopyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBufferImageCopy::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn buffer_offset(mut self, value: VkDeviceSize) -> VkBufferImageCopyBuilder<'a> {
        self.s.buffer_offset = value;
        self
    }

    pub fn buffer_row_length(mut self, value: u32) -> VkBufferImageCopyBuilder<'a> {
        self.s.buffer_row_length = value;
        self
    }

    pub fn buffer_image_height(mut self, value: u32) -> VkBufferImageCopyBuilder<'a> {
        self.s.buffer_image_height = value;
        self
    }

    pub fn image_subresource(mut self, value: VkImageSubresourceLayers) -> VkBufferImageCopyBuilder<'a> {
        self.s.image_subresource = value;
        self
    }

    pub fn image_offset(mut self, value: VkOffset3D) -> VkBufferImageCopyBuilder<'a> {
        self.s.image_offset = value;
        self
    }

    pub fn image_extent(mut self, value: VkExtent3D) -> VkBufferImageCopyBuilder<'a> {
        self.s.image_extent = value;
        self
    }
}

impl<'a> core::ops::Deref for VkBufferImageCopyBuilder<'a> {
    type Target = VkBufferImageCopy;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBufferImageCopyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageBlitBuilder<'a> {
    s: VkImageBlit,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageBlitBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageBlit::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn src_subresource(mut self, value: VkImageSubresourceLayers) -> VkImageBlitBuilder<'a> {
        self.s.src_subresource = value;
        self
    }

    pub fn src_offsets(mut self, value: [VkOffset3D; 2]) -> VkImageBlitBuilder<'a> {
        self.s.src_offsets = value;
        self
    }

    pub fn dst_subresource(mut self, value: VkImageSubresourceLayers) -> VkImageBlitBuilder<'a> {
        self.s.dst_subresource = value;
        self
    }

    pub fn dst_offsets(mut self, value: [VkOffset3D; 2]) -> VkImageBlitBuilder<'a> {
        self.s.dst_offsets = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageBlitBuilder<'a> {
    type Target = VkImageBlit;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageBlitBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageCopyBuilder<'a> {
    s: VkImageCopy,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageCopyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageCopy::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn src_subresource(mut self, value: VkImageSubresourceLayers) -> VkImageCopyBuilder<'a> {
        self.s.src_subresource = value;
        self
    }

    pub fn src_offset(mut self, value: VkOffset3D) -> VkImageCopyBuilder<'a> {
        self.s.src_offset = value;
        self
    }

    pub fn dst_subresource(mut self, value: VkImageSubresourceLayers) -> VkImageCopyBuilder<'a> {
        self.s.dst_subresource = value;
        self
    }

    pub fn dst_offset(mut self, value: VkOffset3D) -> VkImageCopyBuilder<'a> {
        self.s.dst_offset = value;
        self
    }

    pub fn extent(mut self, value: VkExtent3D) -> VkImageCopyBuilder<'a> {
        self.s.extent = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageCopyBuilder<'a> {
    type Target = VkImageCopy;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageCopyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferCopyBuilder<'a> {
    s: VkBufferCopy,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBufferCopyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBufferCopy::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn src_offset(mut self, value: VkDeviceSize) -> VkBufferCopyBuilder<'a> {
        self.s.src_offset = value;
        self
    }

    pub fn dst_offset(mut self, value: VkDeviceSize) -> VkBufferCopyBuilder<'a> {
        self.s.dst_offset = value;
        self
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkBufferCopyBuilder<'a> {
        self.s.size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkBufferCopyBuilder<'a> {
    type Target = VkBufferCopy;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBufferCopyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkViewportBuilder<'a> {
    s: VkViewport,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkViewportBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkViewport::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn x(mut self, value: f32) -> VkViewportBuilder<'a> {
        self.s.x = value;
        self
    }

    pub fn y(mut self, value: f32) -> VkViewportBuilder<'a> {
        self.s.y = value;
        self
    }

    pub fn width(mut self, value: f32) -> VkViewportBuilder<'a> {
        self.s.width = value;
        self
    }

    pub fn height(mut self, value: f32) -> VkViewportBuilder<'a> {
        self.s.height = value;
        self
    }

    pub fn min_depth(mut self, value: f32) -> VkViewportBuilder<'a> {
        self.s.min_depth = value;
        self
    }

    pub fn max_depth(mut self, value: f32) -> VkViewportBuilder<'a> {
        self.s.max_depth = value;
        self
    }
}

impl<'a> core::ops::Deref for VkViewportBuilder<'a> {
    type Target = VkViewport;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkViewportBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandBufferBeginInfoBuilder<'a> {
    s: VkCommandBufferBeginInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkCommandBufferBeginInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkCommandBufferBeginInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkCommandBufferBeginInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsCommandBufferBeginInfo>(mut self, next: &'a mut T) -> VkCommandBufferBeginInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkCommandBufferUsageFlags) -> VkCommandBufferBeginInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn p_inheritance_info(mut self, value: Option<&'a VkCommandBufferInheritanceInfo>) -> VkCommandBufferBeginInfoBuilder<'a> {
        self.s.p_inheritance_info = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }
}

impl<'a> core::ops::Deref for VkCommandBufferBeginInfoBuilder<'a> {
    type Target = VkCommandBufferBeginInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkCommandBufferBeginInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandBufferInheritanceInfoBuilder<'a> {
    s: VkCommandBufferInheritanceInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkCommandBufferInheritanceInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkCommandBufferInheritanceInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsCommandBufferInheritanceInfo>(mut self, next: &'a mut T) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn render_pass(mut self, value: VkRenderPass) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.render_pass = value;
        self
    }

    pub fn subpass(mut self, value: u32) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.subpass = value;
        self
    }

    pub fn framebuffer(mut self, value: VkFramebuffer) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.framebuffer = value;
        self
    }

    pub fn occlusion_query_enable(mut self, value: bool) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.occlusion_query_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn query_flags(mut self, value: VkQueryControlFlags) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.query_flags = value;
        self
    }

    pub fn pipeline_statistics(mut self, value: VkQueryPipelineStatisticFlags) -> VkCommandBufferInheritanceInfoBuilder<'a> {
        self.s.pipeline_statistics = value;
        self
    }
}

impl<'a> core::ops::Deref for VkCommandBufferInheritanceInfoBuilder<'a> {
    type Target = VkCommandBufferInheritanceInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkCommandBufferInheritanceInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandBufferAllocateInfoBuilder<'a> {
    s: VkCommandBufferAllocateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkCommandBufferAllocateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkCommandBufferAllocateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkCommandBufferAllocateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsCommandBufferAllocateInfo>(mut self, next: &'a mut T) -> VkCommandBufferAllocateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn command_pool(mut self, value: VkCommandPool) -> VkCommandBufferAllocateInfoBuilder<'a> {
        self.s.command_pool = value;
        self
    }

    pub fn level(mut self, value: VkCommandBufferLevel) -> VkCommandBufferAllocateInfoBuilder<'a> {
        self.s.level = value;
        self
    }

    pub fn command_buffer_count(mut self, value: u32) -> VkCommandBufferAllocateInfoBuilder<'a> {
        self.s.command_buffer_count = value;
        self
    }
}

impl<'a> core::ops::Deref for VkCommandBufferAllocateInfoBuilder<'a> {
    type Target = VkCommandBufferAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkCommandBufferAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCommandPoolCreateInfoBuilder<'a> {
    s: VkCommandPoolCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkCommandPoolCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkCommandPoolCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkCommandPoolCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsCommandPoolCreateInfo>(mut self, next: &'a mut T) -> VkCommandPoolCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkCommandPoolCreateFlags) -> VkCommandPoolCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn queue_family_index(mut self, value: u32) -> VkCommandPoolCreateInfoBuilder<'a> {
        self.s.queue_family_index = value;
        self
    }
}

impl<'a> core::ops::Deref for VkCommandPoolCreateInfoBuilder<'a> {
    type Target = VkCommandPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkCommandPoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkRenderPassCreateInfoBuilder<'a> {
    s: VkRenderPassCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkRenderPassCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkRenderPassCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsRenderPassCreateInfo>(mut self, next: &'a mut T) -> VkRenderPassCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkRenderPassCreateFlags) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn attachment_count(mut self, value: u32) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.attachment_count = value;
        self
    }

    pub fn p_attachments(mut self, values: &'a [VkAttachmentDescription]) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.attachment_count = values.len() as _;
        self.s.p_attachments = values.as_ptr();
        self
    }

    pub fn subpass_count(mut self, value: u32) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.subpass_count = value;
        self
    }

    pub fn p_subpasses(mut self, values: &'a [VkSubpassDescription]) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.subpass_count = values.len() as _;
        self.s.p_subpasses = values.as_ptr();
        self
    }

    pub fn dependency_count(mut self, value: u32) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.dependency_count = value;
        self
    }

    pub fn p_dependencies(mut self, values: &'a [VkSubpassDependency]) -> VkRenderPassCreateInfoBuilder<'a> {
        self.s.dependency_count = values.len() as _;
        self.s.p_dependencies = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkRenderPassCreateInfoBuilder<'a> {
    type Target = VkRenderPassCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkRenderPassCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubpassDependencyBuilder<'a> {
    s: VkSubpassDependency,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSubpassDependencyBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSubpassDependency::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn src_subpass(mut self, value: u32) -> VkSubpassDependencyBuilder<'a> {
        self.s.src_subpass = value;
        self
    }

    pub fn dst_subpass(mut self, value: u32) -> VkSubpassDependencyBuilder<'a> {
        self.s.dst_subpass = value;
        self
    }

    pub fn src_stage_mask(mut self, value: VkPipelineStageFlags) -> VkSubpassDependencyBuilder<'a> {
        self.s.src_stage_mask = value;
        self
    }

    pub fn dst_stage_mask(mut self, value: VkPipelineStageFlags) -> VkSubpassDependencyBuilder<'a> {
        self.s.dst_stage_mask = value;
        self
    }

    pub fn src_access_mask(mut self, value: VkAccessFlags) -> VkSubpassDependencyBuilder<'a> {
        self.s.src_access_mask = value;
        self
    }

    pub fn dst_access_mask(mut self, value: VkAccessFlags) -> VkSubpassDependencyBuilder<'a> {
        self.s.dst_access_mask = value;
        self
    }

    pub fn dependency_flags(mut self, value: VkDependencyFlags) -> VkSubpassDependencyBuilder<'a> {
        self.s.dependency_flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSubpassDependencyBuilder<'a> {
    type Target = VkSubpassDependency;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSubpassDependencyBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubpassDescriptionBuilder<'a> {
    s: VkSubpassDescription,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSubpassDescriptionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSubpassDescription::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn flags(mut self, value: VkSubpassDescriptionFlags) -> VkSubpassDescriptionBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn pipeline_bind_point(mut self, value: VkPipelineBindPoint) -> VkSubpassDescriptionBuilder<'a> {
        self.s.pipeline_bind_point = value;
        self
    }

    pub fn input_attachment_count(mut self, value: u32) -> VkSubpassDescriptionBuilder<'a> {
        self.s.input_attachment_count = value;
        self
    }

    pub fn p_input_attachments(mut self, values: &'a [VkAttachmentReference]) -> VkSubpassDescriptionBuilder<'a> {
        self.s.input_attachment_count = values.len() as _;
        self.s.p_input_attachments = values.as_ptr();
        self
    }

    pub fn color_attachment_count(mut self, value: u32) -> VkSubpassDescriptionBuilder<'a> {
        self.s.color_attachment_count = value;
        self
    }

    pub fn p_color_attachments(mut self, values: &'a [VkAttachmentReference]) -> VkSubpassDescriptionBuilder<'a> {
        self.s.color_attachment_count = values.len() as _;
        self.s.p_color_attachments = values.as_ptr();
        self
    }

    pub fn p_resolve_attachments(mut self, values: &'a [VkAttachmentReference]) -> VkSubpassDescriptionBuilder<'a> {
        self.s.color_attachment_count = values.len() as _;
        self.s.p_resolve_attachments = values.as_ptr();
        self
    }

    pub fn p_depth_stencil_attachment(mut self, value: Option<&'a VkAttachmentReference>) -> VkSubpassDescriptionBuilder<'a> {
        self.s.p_depth_stencil_attachment = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn preserve_attachment_count(mut self, value: u32) -> VkSubpassDescriptionBuilder<'a> {
        self.s.preserve_attachment_count = value;
        self
    }

    pub fn p_preserve_attachments(mut self, values: &'a [u32]) -> VkSubpassDescriptionBuilder<'a> {
        self.s.preserve_attachment_count = values.len() as _;
        self.s.p_preserve_attachments = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkSubpassDescriptionBuilder<'a> {
    type Target = VkSubpassDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSubpassDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkAttachmentReferenceBuilder<'a> {
    s: VkAttachmentReference,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkAttachmentReferenceBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkAttachmentReference::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn attachment(mut self, value: u32) -> VkAttachmentReferenceBuilder<'a> {
        self.s.attachment = value;
        self
    }

    pub fn layout(mut self, value: VkImageLayout) -> VkAttachmentReferenceBuilder<'a> {
        self.s.layout = value;
        self
    }
}

impl<'a> core::ops::Deref for VkAttachmentReferenceBuilder<'a> {
    type Target = VkAttachmentReference;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkAttachmentReferenceBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkAttachmentDescriptionBuilder<'a> {
    s: VkAttachmentDescription,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkAttachmentDescriptionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkAttachmentDescription::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn flags(mut self, value: VkAttachmentDescriptionFlags) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn format(mut self, value: VkFormat) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.format = value;
        self
    }

    pub fn samples(mut self, value: VkSampleCountFlagBits) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.samples = value;
        self
    }

    pub fn load_op(mut self, value: VkAttachmentLoadOp) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.load_op = value;
        self
    }

    pub fn store_op(mut self, value: VkAttachmentStoreOp) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.store_op = value;
        self
    }

    pub fn stencil_load_op(mut self, value: VkAttachmentLoadOp) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.stencil_load_op = value;
        self
    }

    pub fn stencil_store_op(mut self, value: VkAttachmentStoreOp) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.stencil_store_op = value;
        self
    }

    pub fn initial_layout(mut self, value: VkImageLayout) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.initial_layout = value;
        self
    }

    pub fn final_layout(mut self, value: VkImageLayout) -> VkAttachmentDescriptionBuilder<'a> {
        self.s.final_layout = value;
        self
    }
}

impl<'a> core::ops::Deref for VkAttachmentDescriptionBuilder<'a> {
    type Target = VkAttachmentDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkAttachmentDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkFramebufferCreateInfoBuilder<'a> {
    s: VkFramebufferCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkFramebufferCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkFramebufferCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsFramebufferCreateInfo>(mut self, next: &'a mut T) -> VkFramebufferCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkFramebufferCreateFlags) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn render_pass(mut self, value: VkRenderPass) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.render_pass = value;
        self
    }

    pub fn attachment_count(mut self, value: u32) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.attachment_count = value;
        self
    }

    pub fn p_attachments(mut self, values: &'a [VkImageView]) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.attachment_count = values.len() as _;
        self.s.p_attachments = values.as_ptr();
        self
    }

    pub fn width(mut self, value: u32) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.width = value;
        self
    }

    pub fn height(mut self, value: u32) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.height = value;
        self
    }

    pub fn layers(mut self, value: u32) -> VkFramebufferCreateInfoBuilder<'a> {
        self.s.layers = value;
        self
    }
}

impl<'a> core::ops::Deref for VkFramebufferCreateInfoBuilder<'a> {
    type Target = VkFramebufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkFramebufferCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkCopyDescriptorSetBuilder<'a> {
    s: VkCopyDescriptorSet,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkCopyDescriptorSetBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkCopyDescriptorSet::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsCopyDescriptorSet>(mut self, next: &'a mut T) -> VkCopyDescriptorSetBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn src_set(mut self, value: VkDescriptorSet) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.src_set = value;
        self
    }

    pub fn src_binding(mut self, value: u32) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.src_binding = value;
        self
    }

    pub fn src_array_element(mut self, value: u32) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.src_array_element = value;
        self
    }

    pub fn dst_set(mut self, value: VkDescriptorSet) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.dst_set = value;
        self
    }

    pub fn dst_binding(mut self, value: u32) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.dst_binding = value;
        self
    }

    pub fn dst_array_element(mut self, value: u32) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.dst_array_element = value;
        self
    }

    pub fn descriptor_count(mut self, value: u32) -> VkCopyDescriptorSetBuilder<'a> {
        self.s.descriptor_count = value;
        self
    }
}

impl<'a> core::ops::Deref for VkCopyDescriptorSetBuilder<'a> {
    type Target = VkCopyDescriptorSet;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkCopyDescriptorSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkWriteDescriptorSetBuilder<'a> {
    s: VkWriteDescriptorSet,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkWriteDescriptorSetBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkWriteDescriptorSet::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsWriteDescriptorSet>(mut self, next: &'a mut T) -> VkWriteDescriptorSetBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn dst_set(mut self, value: VkDescriptorSet) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.dst_set = value;
        self
    }

    pub fn dst_binding(mut self, value: u32) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.dst_binding = value;
        self
    }

    pub fn dst_array_element(mut self, value: u32) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.dst_array_element = value;
        self
    }

    pub fn descriptor_count(mut self, value: u32) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.descriptor_count = value;
        self
    }

    pub fn descriptor_type(mut self, value: VkDescriptorType) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.descriptor_type = value;
        self
    }

    pub fn p_image_info(mut self, values: &'a [VkDescriptorImageInfo]) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.descriptor_count = values.len() as _;
        self.s.p_image_info = values.as_ptr();
        self
    }

    pub fn p_buffer_info(mut self, values: &'a [VkDescriptorBufferInfo]) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.descriptor_count = values.len() as _;
        self.s.p_buffer_info = values.as_ptr();
        self
    }

    pub fn p_texel_buffer_view(mut self, values: &'a [VkBufferView]) -> VkWriteDescriptorSetBuilder<'a> {
        self.s.descriptor_count = values.len() as _;
        self.s.p_texel_buffer_view = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkWriteDescriptorSetBuilder<'a> {
    type Target = VkWriteDescriptorSet;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkWriteDescriptorSetBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorBufferInfoBuilder<'a> {
    s: VkDescriptorBufferInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorBufferInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorBufferInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn buffer(mut self, value: VkBuffer) -> VkDescriptorBufferInfoBuilder<'a> {
        self.s.buffer = value;
        self
    }

    pub fn offset(mut self, value: VkDeviceSize) -> VkDescriptorBufferInfoBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn range(mut self, value: VkDeviceSize) -> VkDescriptorBufferInfoBuilder<'a> {
        self.s.range = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorBufferInfoBuilder<'a> {
    type Target = VkDescriptorBufferInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorBufferInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorImageInfoBuilder<'a> {
    s: VkDescriptorImageInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorImageInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorImageInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn sampler(mut self, value: VkSampler) -> VkDescriptorImageInfoBuilder<'a> {
        self.s.sampler = value;
        self
    }

    pub fn image_view(mut self, value: VkImageView) -> VkDescriptorImageInfoBuilder<'a> {
        self.s.image_view = value;
        self
    }

    pub fn image_layout(mut self, value: VkImageLayout) -> VkDescriptorImageInfoBuilder<'a> {
        self.s.image_layout = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorImageInfoBuilder<'a> {
    type Target = VkDescriptorImageInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorImageInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorSetAllocateInfoBuilder<'a> {
    s: VkDescriptorSetAllocateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorSetAllocateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorSetAllocateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDescriptorSetAllocateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDescriptorSetAllocateInfo>(mut self, next: &'a mut T) -> VkDescriptorSetAllocateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn descriptor_pool(mut self, value: VkDescriptorPool) -> VkDescriptorSetAllocateInfoBuilder<'a> {
        self.s.descriptor_pool = value;
        self
    }

    pub fn descriptor_set_count(mut self, value: u32) -> VkDescriptorSetAllocateInfoBuilder<'a> {
        self.s.descriptor_set_count = value;
        self
    }

    pub fn p_set_layouts(mut self, values: &'a [VkDescriptorSetLayout]) -> VkDescriptorSetAllocateInfoBuilder<'a> {
        self.s.descriptor_set_count = values.len() as _;
        self.s.p_set_layouts = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorSetAllocateInfoBuilder<'a> {
    type Target = VkDescriptorSetAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorSetAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorPoolCreateInfoBuilder<'a> {
    s: VkDescriptorPoolCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorPoolCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorPoolCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDescriptorPoolCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDescriptorPoolCreateInfo>(mut self, next: &'a mut T) -> VkDescriptorPoolCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkDescriptorPoolCreateFlags) -> VkDescriptorPoolCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn max_sets(mut self, value: u32) -> VkDescriptorPoolCreateInfoBuilder<'a> {
        self.s.max_sets = value;
        self
    }

    pub fn pool_size_count(mut self, value: u32) -> VkDescriptorPoolCreateInfoBuilder<'a> {
        self.s.pool_size_count = value;
        self
    }

    pub fn p_pool_sizes(mut self, values: &'a [VkDescriptorPoolSize]) -> VkDescriptorPoolCreateInfoBuilder<'a> {
        self.s.pool_size_count = values.len() as _;
        self.s.p_pool_sizes = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorPoolCreateInfoBuilder<'a> {
    type Target = VkDescriptorPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorPoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorPoolSizeBuilder<'a> {
    s: VkDescriptorPoolSize,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorPoolSizeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorPoolSize::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn kind(mut self, value: VkDescriptorType) -> VkDescriptorPoolSizeBuilder<'a> {
        self.s.kind = value;
        self
    }

    pub fn descriptor_count(mut self, value: u32) -> VkDescriptorPoolSizeBuilder<'a> {
        self.s.descriptor_count = value;
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorPoolSizeBuilder<'a> {
    type Target = VkDescriptorPoolSize;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorPoolSizeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorSetLayoutCreateInfoBuilder<'a> {
    s: VkDescriptorSetLayoutCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorSetLayoutCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorSetLayoutCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDescriptorSetLayoutCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDescriptorSetLayoutCreateInfo>(mut self, next: &'a mut T) -> VkDescriptorSetLayoutCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkDescriptorSetLayoutCreateFlags) -> VkDescriptorSetLayoutCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn binding_count(mut self, value: u32) -> VkDescriptorSetLayoutCreateInfoBuilder<'a> {
        self.s.binding_count = value;
        self
    }

    pub fn p_bindings(mut self, values: &'a [VkDescriptorSetLayoutBinding]) -> VkDescriptorSetLayoutCreateInfoBuilder<'a> {
        self.s.binding_count = values.len() as _;
        self.s.p_bindings = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorSetLayoutCreateInfoBuilder<'a> {
    type Target = VkDescriptorSetLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorSetLayoutCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDescriptorSetLayoutBindingBuilder<'a> {
    s: VkDescriptorSetLayoutBinding,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDescriptorSetLayoutBindingBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDescriptorSetLayoutBinding::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn binding(mut self, value: u32) -> VkDescriptorSetLayoutBindingBuilder<'a> {
        self.s.binding = value;
        self
    }

    pub fn descriptor_type(mut self, value: VkDescriptorType) -> VkDescriptorSetLayoutBindingBuilder<'a> {
        self.s.descriptor_type = value;
        self
    }

    pub fn descriptor_count(mut self, value: u32) -> VkDescriptorSetLayoutBindingBuilder<'a> {
        self.s.descriptor_count = value;
        self
    }

    pub fn stage_flags(mut self, value: VkShaderStageFlags) -> VkDescriptorSetLayoutBindingBuilder<'a> {
        self.s.stage_flags = value;
        self
    }

    pub fn p_immutable_samplers(mut self, values: &'a [VkSampler]) -> VkDescriptorSetLayoutBindingBuilder<'a> {
        self.s.descriptor_count = values.len() as _;
        self.s.p_immutable_samplers = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDescriptorSetLayoutBindingBuilder<'a> {
    type Target = VkDescriptorSetLayoutBinding;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDescriptorSetLayoutBindingBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSamplerCreateInfoBuilder<'a> {
    s: VkSamplerCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSamplerCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSamplerCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsSamplerCreateInfo>(mut self, next: &'a mut T) -> VkSamplerCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkSamplerCreateFlags) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn mag_filter(mut self, value: VkFilter) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.mag_filter = value;
        self
    }

    pub fn min_filter(mut self, value: VkFilter) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.min_filter = value;
        self
    }

    pub fn mipmap_mode(mut self, value: VkSamplerMipmapMode) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.mipmap_mode = value;
        self
    }

    pub fn address_mode_u(mut self, value: VkSamplerAddressMode) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.address_mode_u = value;
        self
    }

    pub fn address_mode_v(mut self, value: VkSamplerAddressMode) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.address_mode_v = value;
        self
    }

    pub fn address_mode_w(mut self, value: VkSamplerAddressMode) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.address_mode_w = value;
        self
    }

    pub fn mip_lod_bias(mut self, value: f32) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.mip_lod_bias = value;
        self
    }

    pub fn anisotropy_enable(mut self, value: bool) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.anisotropy_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn max_anisotropy(mut self, value: f32) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.max_anisotropy = value;
        self
    }

    pub fn compare_enable(mut self, value: bool) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.compare_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn compare_op(mut self, value: VkCompareOp) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.compare_op = value;
        self
    }

    pub fn min_lod(mut self, value: f32) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.min_lod = value;
        self
    }

    pub fn max_lod(mut self, value: f32) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.max_lod = value;
        self
    }

    pub fn border_color(mut self, value: VkBorderColor) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.border_color = value;
        self
    }

    pub fn unnormalized_coordinates(mut self, value: bool) -> VkSamplerCreateInfoBuilder<'a> {
        self.s.unnormalized_coordinates = if value { VK_TRUE } else { VK_FALSE };
        self
    }
}

impl<'a> core::ops::Deref for VkSamplerCreateInfoBuilder<'a> {
    type Target = VkSamplerCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSamplerCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineLayoutCreateInfoBuilder<'a> {
    s: VkPipelineLayoutCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineLayoutCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineLayoutCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineLayoutCreateInfo>(mut self, next: &'a mut T) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineLayoutCreateFlags) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn set_layout_count(mut self, value: u32) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        self.s.set_layout_count = value;
        self
    }

    pub fn p_set_layouts(mut self, values: &'a [VkDescriptorSetLayout]) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        self.s.set_layout_count = values.len() as _;
        self.s.p_set_layouts = values.as_ptr();
        self
    }

    pub fn push_constant_range_count(mut self, value: u32) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        self.s.push_constant_range_count = value;
        self
    }

    pub fn p_push_constant_ranges(mut self, values: &'a [VkPushConstantRange]) -> VkPipelineLayoutCreateInfoBuilder<'a> {
        self.s.push_constant_range_count = values.len() as _;
        self.s.p_push_constant_ranges = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineLayoutCreateInfoBuilder<'a> {
    type Target = VkPipelineLayoutCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineLayoutCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPushConstantRangeBuilder<'a> {
    s: VkPushConstantRange,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPushConstantRangeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPushConstantRange::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn stage_flags(mut self, value: VkShaderStageFlags) -> VkPushConstantRangeBuilder<'a> {
        self.s.stage_flags = value;
        self
    }

    pub fn offset(mut self, value: u32) -> VkPushConstantRangeBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn size(mut self, value: u32) -> VkPushConstantRangeBuilder<'a> {
        self.s.size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPushConstantRangeBuilder<'a> {
    type Target = VkPushConstantRange;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPushConstantRangeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkComputePipelineCreateInfoBuilder<'a> {
    s: VkComputePipelineCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkComputePipelineCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkComputePipelineCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkComputePipelineCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsComputePipelineCreateInfo>(mut self, next: &'a mut T) -> VkComputePipelineCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineCreateFlags) -> VkComputePipelineCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn stage(mut self, value: VkPipelineShaderStageCreateInfo) -> VkComputePipelineCreateInfoBuilder<'a> {
        self.s.stage = value;
        self
    }

    pub fn layout(mut self, value: VkPipelineLayout) -> VkComputePipelineCreateInfoBuilder<'a> {
        self.s.layout = value;
        self
    }

    pub fn base_pipeline_handle(mut self, value: VkPipeline) -> VkComputePipelineCreateInfoBuilder<'a> {
        self.s.base_pipeline_handle = value;
        self
    }

    pub fn base_pipeline_index(mut self, value: i32) -> VkComputePipelineCreateInfoBuilder<'a> {
        self.s.base_pipeline_index = value;
        self
    }
}

impl<'a> core::ops::Deref for VkComputePipelineCreateInfoBuilder<'a> {
    type Target = VkComputePipelineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkComputePipelineCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineShaderStageCreateInfoBuilder<'a> {
    s: VkPipelineShaderStageCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineShaderStageCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineShaderStageCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineShaderStageCreateInfo>(mut self, next: &'a mut T) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineShaderStageCreateFlags) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn stage(mut self, value: VkShaderStageFlagBits) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        self.s.stage = value;
        self
    }

    pub fn module(mut self, value: VkShaderModule) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        self.s.module = value;
        self
    }

    pub fn p_name(mut self, values: &'a [u8]) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        
        self.s.p_name = values.as_ptr();
        self
    }

    pub fn p_specialization_info(mut self, value: Option<&'a VkSpecializationInfo>) -> VkPipelineShaderStageCreateInfoBuilder<'a> {
        self.s.p_specialization_info = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineShaderStageCreateInfoBuilder<'a> {
    type Target = VkPipelineShaderStageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineShaderStageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSpecializationInfoBuilder<'a> {
    s: VkSpecializationInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSpecializationInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSpecializationInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn map_entry_count(mut self, value: u32) -> VkSpecializationInfoBuilder<'a> {
        self.s.map_entry_count = value;
        self
    }

    pub fn p_map_entries(mut self, values: &'a [VkSpecializationMapEntry]) -> VkSpecializationInfoBuilder<'a> {
        self.s.map_entry_count = values.len() as _;
        self.s.p_map_entries = values.as_ptr();
        self
    }

    pub fn data_size(mut self, value: usize) -> VkSpecializationInfoBuilder<'a> {
        self.s.data_size = value;
        self
    }

    pub fn p_data(mut self, values: &'a [core::ffi::c_void]) -> VkSpecializationInfoBuilder<'a> {
        self.s.data_size = values.len() as _;
        self.s.p_data = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkSpecializationInfoBuilder<'a> {
    type Target = VkSpecializationInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSpecializationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSpecializationMapEntryBuilder<'a> {
    s: VkSpecializationMapEntry,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSpecializationMapEntryBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSpecializationMapEntry::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn ant_id(mut self, value: u32) -> VkSpecializationMapEntryBuilder<'a> {
        self.s.ant_id = value;
        self
    }

    pub fn offset(mut self, value: u32) -> VkSpecializationMapEntryBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn size(mut self, value: usize) -> VkSpecializationMapEntryBuilder<'a> {
        self.s.size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSpecializationMapEntryBuilder<'a> {
    type Target = VkSpecializationMapEntry;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSpecializationMapEntryBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkGraphicsPipelineCreateInfoBuilder<'a> {
    s: VkGraphicsPipelineCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkGraphicsPipelineCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkGraphicsPipelineCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsGraphicsPipelineCreateInfo>(mut self, next: &'a mut T) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineCreateFlags) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn stage_count(mut self, value: u32) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.stage_count = value;
        self
    }

    pub fn p_stages(mut self, values: &'a [VkPipelineShaderStageCreateInfo]) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.stage_count = values.len() as _;
        self.s.p_stages = values.as_ptr();
        self
    }

    pub fn p_vertex_input_state(mut self, value: Option<&'a VkPipelineVertexInputStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_vertex_input_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_input_assembly_state(mut self, value: Option<&'a VkPipelineInputAssemblyStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_input_assembly_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_tessellation_state(mut self, value: Option<&'a VkPipelineTessellationStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_tessellation_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_viewport_state(mut self, value: Option<&'a VkPipelineViewportStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_viewport_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_rasterization_state(mut self, value: &'a VkPipelineRasterizationStateCreateInfo) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_rasterization_state = value;
        self
    }

    pub fn p_multisample_state(mut self, value: Option<&'a VkPipelineMultisampleStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_multisample_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_depth_stencil_state(mut self, value: Option<&'a VkPipelineDepthStencilStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_depth_stencil_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_color_blend_state(mut self, value: Option<&'a VkPipelineColorBlendStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_color_blend_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn p_dynamic_state(mut self, value: Option<&'a VkPipelineDynamicStateCreateInfo>) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.p_dynamic_state = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn layout(mut self, value: VkPipelineLayout) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.layout = value;
        self
    }

    pub fn render_pass(mut self, value: VkRenderPass) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.render_pass = value;
        self
    }

    pub fn subpass(mut self, value: u32) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.subpass = value;
        self
    }

    pub fn base_pipeline_handle(mut self, value: VkPipeline) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.base_pipeline_handle = value;
        self
    }

    pub fn base_pipeline_index(mut self, value: i32) -> VkGraphicsPipelineCreateInfoBuilder<'a> {
        self.s.base_pipeline_index = value;
        self
    }
}

impl<'a> core::ops::Deref for VkGraphicsPipelineCreateInfoBuilder<'a> {
    type Target = VkGraphicsPipelineCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkGraphicsPipelineCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineDynamicStateCreateInfoBuilder<'a> {
    s: VkPipelineDynamicStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineDynamicStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineDynamicStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineDynamicStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineDynamicStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineDynamicStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineDynamicStateCreateFlags) -> VkPipelineDynamicStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn dynamic_state_count(mut self, value: u32) -> VkPipelineDynamicStateCreateInfoBuilder<'a> {
        self.s.dynamic_state_count = value;
        self
    }

    pub fn p_dynamic_states(mut self, values: &'a [VkDynamicState]) -> VkPipelineDynamicStateCreateInfoBuilder<'a> {
        self.s.dynamic_state_count = values.len() as _;
        self.s.p_dynamic_states = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineDynamicStateCreateInfoBuilder<'a> {
    type Target = VkPipelineDynamicStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineDynamicStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineColorBlendStateCreateInfoBuilder<'a> {
    s: VkPipelineColorBlendStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineColorBlendStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineColorBlendStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineColorBlendStateCreateFlags) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn logic_op_enable(mut self, value: bool) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.logic_op_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn logic_op(mut self, value: VkLogicOp) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.logic_op = value;
        self
    }

    pub fn attachment_count(mut self, value: u32) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.attachment_count = value;
        self
    }

    pub fn p_attachments(mut self, values: &'a [VkPipelineColorBlendAttachmentState]) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.attachment_count = values.len() as _;
        self.s.p_attachments = values.as_ptr();
        self
    }

    pub fn blend_constants(mut self, value: [f32; 4]) -> VkPipelineColorBlendStateCreateInfoBuilder<'a> {
        self.s.blend_constants = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineColorBlendStateCreateInfoBuilder<'a> {
    type Target = VkPipelineColorBlendStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineColorBlendStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineColorBlendAttachmentStateBuilder<'a> {
    s: VkPipelineColorBlendAttachmentState,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineColorBlendAttachmentStateBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineColorBlendAttachmentState::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn blend_enable(mut self, value: bool) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.blend_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn src_color_blend_factor(mut self, value: VkBlendFactor) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.src_color_blend_factor = value;
        self
    }

    pub fn dst_color_blend_factor(mut self, value: VkBlendFactor) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.dst_color_blend_factor = value;
        self
    }

    pub fn color_blend_op(mut self, value: VkBlendOp) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.color_blend_op = value;
        self
    }

    pub fn src_alpha_blend_factor(mut self, value: VkBlendFactor) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.src_alpha_blend_factor = value;
        self
    }

    pub fn dst_alpha_blend_factor(mut self, value: VkBlendFactor) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.dst_alpha_blend_factor = value;
        self
    }

    pub fn alpha_blend_op(mut self, value: VkBlendOp) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.alpha_blend_op = value;
        self
    }

    pub fn color_write_mask(mut self, value: VkColorComponentFlags) -> VkPipelineColorBlendAttachmentStateBuilder<'a> {
        self.s.color_write_mask = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineColorBlendAttachmentStateBuilder<'a> {
    type Target = VkPipelineColorBlendAttachmentState;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineColorBlendAttachmentStateBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
    s: VkPipelineDepthStencilStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineDepthStencilStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineDepthStencilStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineDepthStencilStateCreateFlags) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn depth_test_enable(mut self, value: bool) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.depth_test_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn depth_write_enable(mut self, value: bool) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.depth_write_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn depth_compare_op(mut self, value: VkCompareOp) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.depth_compare_op = value;
        self
    }

    pub fn depth_bounds_test_enable(mut self, value: bool) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.depth_bounds_test_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn stencil_test_enable(mut self, value: bool) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.stencil_test_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn front(mut self, value: VkStencilOpState) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.front = value;
        self
    }

    pub fn back(mut self, value: VkStencilOpState) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.back = value;
        self
    }

    pub fn min_depth_bounds(mut self, value: f32) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.min_depth_bounds = value;
        self
    }

    pub fn max_depth_bounds(mut self, value: f32) -> VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
        self.s.max_depth_bounds = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
    type Target = VkPipelineDepthStencilStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineDepthStencilStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkStencilOpStateBuilder<'a> {
    s: VkStencilOpState,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkStencilOpStateBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkStencilOpState::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn fail_op(mut self, value: VkStencilOp) -> VkStencilOpStateBuilder<'a> {
        self.s.fail_op = value;
        self
    }

    pub fn pass_op(mut self, value: VkStencilOp) -> VkStencilOpStateBuilder<'a> {
        self.s.pass_op = value;
        self
    }

    pub fn depth_fail_op(mut self, value: VkStencilOp) -> VkStencilOpStateBuilder<'a> {
        self.s.depth_fail_op = value;
        self
    }

    pub fn compare_op(mut self, value: VkCompareOp) -> VkStencilOpStateBuilder<'a> {
        self.s.compare_op = value;
        self
    }

    pub fn compare_mask(mut self, value: u32) -> VkStencilOpStateBuilder<'a> {
        self.s.compare_mask = value;
        self
    }

    pub fn write_mask(mut self, value: u32) -> VkStencilOpStateBuilder<'a> {
        self.s.write_mask = value;
        self
    }

    pub fn reference(mut self, value: u32) -> VkStencilOpStateBuilder<'a> {
        self.s.reference = value;
        self
    }
}

impl<'a> core::ops::Deref for VkStencilOpStateBuilder<'a> {
    type Target = VkStencilOpState;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkStencilOpStateBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineMultisampleStateCreateInfoBuilder<'a> {
    s: VkPipelineMultisampleStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineMultisampleStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineMultisampleStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineMultisampleStateCreateFlags) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn rasterization_samples(mut self, value: VkSampleCountFlagBits) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.rasterization_samples = value;
        self
    }

    pub fn sample_shading_enable(mut self, value: bool) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.sample_shading_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn min_sample_shading(mut self, value: f32) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.min_sample_shading = value;
        self
    }

    pub fn p_sample_mask(mut self, values: &'a [VkSampleMask]) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        
        self.s.p_sample_mask = values.as_ptr();
        self
    }

    pub fn alpha_to_coverage_enable(mut self, value: bool) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.alpha_to_coverage_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn alpha_to_one_enable(mut self, value: bool) -> VkPipelineMultisampleStateCreateInfoBuilder<'a> {
        self.s.alpha_to_one_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineMultisampleStateCreateInfoBuilder<'a> {
    type Target = VkPipelineMultisampleStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineMultisampleStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineRasterizationStateCreateInfoBuilder<'a> {
    s: VkPipelineRasterizationStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineRasterizationStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineRasterizationStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineRasterizationStateCreateFlags) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn depth_clamp_enable(mut self, value: bool) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.depth_clamp_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn rasterizer_discard_enable(mut self, value: bool) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.rasterizer_discard_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn polygon_mode(mut self, value: VkPolygonMode) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.polygon_mode = value;
        self
    }

    pub fn cull_mode(mut self, value: VkCullModeFlags) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.cull_mode = value;
        self
    }

    pub fn front_face(mut self, value: VkFrontFace) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.front_face = value;
        self
    }

    pub fn depth_bias_enable(mut self, value: bool) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.depth_bias_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn depth_bias_constant_factor(mut self, value: f32) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.depth_bias_constant_factor = value;
        self
    }

    pub fn depth_bias_clamp(mut self, value: f32) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.depth_bias_clamp = value;
        self
    }

    pub fn depth_bias_slope_factor(mut self, value: f32) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.depth_bias_slope_factor = value;
        self
    }

    pub fn line_width(mut self, value: f32) -> VkPipelineRasterizationStateCreateInfoBuilder<'a> {
        self.s.line_width = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineRasterizationStateCreateInfoBuilder<'a> {
    type Target = VkPipelineRasterizationStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineRasterizationStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineViewportStateCreateInfoBuilder<'a> {
    s: VkPipelineViewportStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineViewportStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineViewportStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineViewportStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineViewportStateCreateFlags) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn viewport_count(mut self, value: u32) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        self.s.viewport_count = value;
        self
    }

    pub fn p_viewports(mut self, values: &'a [VkViewport]) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        self.s.viewport_count = values.len() as _;
        self.s.p_viewports = values.as_ptr();
        self
    }

    pub fn scissor_count(mut self, value: u32) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        self.s.scissor_count = value;
        self
    }

    pub fn p_scissors(mut self, values: &'a [VkRect2D]) -> VkPipelineViewportStateCreateInfoBuilder<'a> {
        self.s.scissor_count = values.len() as _;
        self.s.p_scissors = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineViewportStateCreateInfoBuilder<'a> {
    type Target = VkPipelineViewportStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineViewportStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineTessellationStateCreateInfoBuilder<'a> {
    s: VkPipelineTessellationStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineTessellationStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineTessellationStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineTessellationStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineTessellationStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineTessellationStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineTessellationStateCreateFlags) -> VkPipelineTessellationStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn patch_control_points(mut self, value: u32) -> VkPipelineTessellationStateCreateInfoBuilder<'a> {
        self.s.patch_control_points = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineTessellationStateCreateInfoBuilder<'a> {
    type Target = VkPipelineTessellationStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineTessellationStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
    s: VkPipelineInputAssemblyStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineInputAssemblyStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineInputAssemblyStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineInputAssemblyStateCreateFlags) -> VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn topology(mut self, value: VkPrimitiveTopology) -> VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
        self.s.topology = value;
        self
    }

    pub fn primitive_restart_enable(mut self, value: bool) -> VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
        self.s.primitive_restart_enable = if value { VK_TRUE } else { VK_FALSE };
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
    type Target = VkPipelineInputAssemblyStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineInputAssemblyStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineVertexInputStateCreateInfoBuilder<'a> {
    s: VkPipelineVertexInputStateCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineVertexInputStateCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineVertexInputStateCreateInfo>(mut self, next: &'a mut T) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineVertexInputStateCreateFlags) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn vertex_binding_description_count(mut self, value: u32) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        self.s.vertex_binding_description_count = value;
        self
    }

    pub fn p_vertex_binding_descriptions(mut self, values: &'a [VkVertexInputBindingDescription]) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        self.s.vertex_binding_description_count = values.len() as _;
        self.s.p_vertex_binding_descriptions = values.as_ptr();
        self
    }

    pub fn vertex_attribute_description_count(mut self, value: u32) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        self.s.vertex_attribute_description_count = value;
        self
    }

    pub fn p_vertex_attribute_descriptions(mut self, values: &'a [VkVertexInputAttributeDescription]) -> VkPipelineVertexInputStateCreateInfoBuilder<'a> {
        self.s.vertex_attribute_description_count = values.len() as _;
        self.s.p_vertex_attribute_descriptions = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineVertexInputStateCreateInfoBuilder<'a> {
    type Target = VkPipelineVertexInputStateCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineVertexInputStateCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkVertexInputAttributeDescriptionBuilder<'a> {
    s: VkVertexInputAttributeDescription,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkVertexInputAttributeDescriptionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkVertexInputAttributeDescription::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn location(mut self, value: u32) -> VkVertexInputAttributeDescriptionBuilder<'a> {
        self.s.location = value;
        self
    }

    pub fn binding(mut self, value: u32) -> VkVertexInputAttributeDescriptionBuilder<'a> {
        self.s.binding = value;
        self
    }

    pub fn format(mut self, value: VkFormat) -> VkVertexInputAttributeDescriptionBuilder<'a> {
        self.s.format = value;
        self
    }

    pub fn offset(mut self, value: u32) -> VkVertexInputAttributeDescriptionBuilder<'a> {
        self.s.offset = value;
        self
    }
}

impl<'a> core::ops::Deref for VkVertexInputAttributeDescriptionBuilder<'a> {
    type Target = VkVertexInputAttributeDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkVertexInputAttributeDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkVertexInputBindingDescriptionBuilder<'a> {
    s: VkVertexInputBindingDescription,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkVertexInputBindingDescriptionBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkVertexInputBindingDescription::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn binding(mut self, value: u32) -> VkVertexInputBindingDescriptionBuilder<'a> {
        self.s.binding = value;
        self
    }

    pub fn stride(mut self, value: u32) -> VkVertexInputBindingDescriptionBuilder<'a> {
        self.s.stride = value;
        self
    }

    pub fn input_rate(mut self, value: VkVertexInputRate) -> VkVertexInputBindingDescriptionBuilder<'a> {
        self.s.input_rate = value;
        self
    }
}

impl<'a> core::ops::Deref for VkVertexInputBindingDescriptionBuilder<'a> {
    type Target = VkVertexInputBindingDescription;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkVertexInputBindingDescriptionBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPipelineCacheCreateInfoBuilder<'a> {
    s: VkPipelineCacheCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPipelineCacheCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPipelineCacheCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkPipelineCacheCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsPipelineCacheCreateInfo>(mut self, next: &'a mut T) -> VkPipelineCacheCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkPipelineCacheCreateFlags) -> VkPipelineCacheCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn initial_data_size(mut self, value: usize) -> VkPipelineCacheCreateInfoBuilder<'a> {
        self.s.initial_data_size = value;
        self
    }

    pub fn p_initial_data(mut self, values: &'a [core::ffi::c_void]) -> VkPipelineCacheCreateInfoBuilder<'a> {
        self.s.initial_data_size = values.len() as _;
        self.s.p_initial_data = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkPipelineCacheCreateInfoBuilder<'a> {
    type Target = VkPipelineCacheCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPipelineCacheCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkShaderModuleCreateInfoBuilder<'a> {
    s: VkShaderModuleCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkShaderModuleCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkShaderModuleCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkShaderModuleCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsShaderModuleCreateInfo>(mut self, next: &'a mut T) -> VkShaderModuleCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkShaderModuleCreateFlags) -> VkShaderModuleCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn code_size(mut self, value: usize) -> VkShaderModuleCreateInfoBuilder<'a> {
        self.s.code_size = value;
        self
    }

    pub fn p_code(mut self, values: &'a [u32]) -> VkShaderModuleCreateInfoBuilder<'a> {
        self.s.code_size = (values.len() * 4) as _;
        self.s.p_code = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkShaderModuleCreateInfoBuilder<'a> {
    type Target = VkShaderModuleCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkShaderModuleCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageViewCreateInfoBuilder<'a> {
    s: VkImageViewCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageViewCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageViewCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsImageViewCreateInfo>(mut self, next: &'a mut T) -> VkImageViewCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkImageViewCreateFlags) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn image(mut self, value: VkImage) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.image = value;
        self
    }

    pub fn view_type(mut self, value: VkImageViewType) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.view_type = value;
        self
    }

    pub fn format(mut self, value: VkFormat) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.format = value;
        self
    }

    pub fn components(mut self, value: VkComponentMapping) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.components = value;
        self
    }

    pub fn subresource_range(mut self, value: VkImageSubresourceRange) -> VkImageViewCreateInfoBuilder<'a> {
        self.s.subresource_range = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageViewCreateInfoBuilder<'a> {
    type Target = VkImageViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageViewCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkComponentMappingBuilder<'a> {
    s: VkComponentMapping,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkComponentMappingBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkComponentMapping::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn r(mut self, value: VkComponentSwizzle) -> VkComponentMappingBuilder<'a> {
        self.s.r = value;
        self
    }

    pub fn g(mut self, value: VkComponentSwizzle) -> VkComponentMappingBuilder<'a> {
        self.s.g = value;
        self
    }

    pub fn b(mut self, value: VkComponentSwizzle) -> VkComponentMappingBuilder<'a> {
        self.s.b = value;
        self
    }

    pub fn a(mut self, value: VkComponentSwizzle) -> VkComponentMappingBuilder<'a> {
        self.s.a = value;
        self
    }
}

impl<'a> core::ops::Deref for VkComponentMappingBuilder<'a> {
    type Target = VkComponentMapping;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkComponentMappingBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubresourceLayoutBuilder<'a> {
    s: VkSubresourceLayout,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSubresourceLayoutBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSubresourceLayout::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn offset(mut self, value: VkDeviceSize) -> VkSubresourceLayoutBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkSubresourceLayoutBuilder<'a> {
        self.s.size = value;
        self
    }

    pub fn row_pitch(mut self, value: VkDeviceSize) -> VkSubresourceLayoutBuilder<'a> {
        self.s.row_pitch = value;
        self
    }

    pub fn array_pitch(mut self, value: VkDeviceSize) -> VkSubresourceLayoutBuilder<'a> {
        self.s.array_pitch = value;
        self
    }

    pub fn depth_pitch(mut self, value: VkDeviceSize) -> VkSubresourceLayoutBuilder<'a> {
        self.s.depth_pitch = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSubresourceLayoutBuilder<'a> {
    type Target = VkSubresourceLayout;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSubresourceLayoutBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageSubresourceBuilder<'a> {
    s: VkImageSubresource,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageSubresourceBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageSubresource::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn aspect_mask(mut self, value: VkImageAspectFlags) -> VkImageSubresourceBuilder<'a> {
        self.s.aspect_mask = value;
        self
    }

    pub fn mip_level(mut self, value: u32) -> VkImageSubresourceBuilder<'a> {
        self.s.mip_level = value;
        self
    }

    pub fn array_layer(mut self, value: u32) -> VkImageSubresourceBuilder<'a> {
        self.s.array_layer = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageSubresourceBuilder<'a> {
    type Target = VkImageSubresource;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageSubresourceBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageCreateInfoBuilder<'a> {
    s: VkImageCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkImageCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsImageCreateInfo>(mut self, next: &'a mut T) -> VkImageCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkImageCreateFlags) -> VkImageCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn image_type(mut self, value: VkImageType) -> VkImageCreateInfoBuilder<'a> {
        self.s.image_type = value;
        self
    }

    pub fn format(mut self, value: VkFormat) -> VkImageCreateInfoBuilder<'a> {
        self.s.format = value;
        self
    }

    pub fn extent(mut self, value: VkExtent3D) -> VkImageCreateInfoBuilder<'a> {
        self.s.extent = value;
        self
    }

    pub fn mip_levels(mut self, value: u32) -> VkImageCreateInfoBuilder<'a> {
        self.s.mip_levels = value;
        self
    }

    pub fn array_layers(mut self, value: u32) -> VkImageCreateInfoBuilder<'a> {
        self.s.array_layers = value;
        self
    }

    pub fn samples(mut self, value: VkSampleCountFlagBits) -> VkImageCreateInfoBuilder<'a> {
        self.s.samples = value;
        self
    }

    pub fn tiling(mut self, value: VkImageTiling) -> VkImageCreateInfoBuilder<'a> {
        self.s.tiling = value;
        self
    }

    pub fn usage(mut self, value: VkImageUsageFlags) -> VkImageCreateInfoBuilder<'a> {
        self.s.usage = value;
        self
    }

    pub fn sharing_mode(mut self, value: VkSharingMode) -> VkImageCreateInfoBuilder<'a> {
        self.s.sharing_mode = value;
        self
    }

    pub fn queue_family_index_count(mut self, value: u32) -> VkImageCreateInfoBuilder<'a> {
        self.s.queue_family_index_count = value;
        self
    }

    pub fn p_queue_family_indices(mut self, values: &'a [u32]) -> VkImageCreateInfoBuilder<'a> {
        self.s.queue_family_index_count = values.len() as _;
        self.s.p_queue_family_indices = values.as_ptr();
        self
    }

    pub fn initial_layout(mut self, value: VkImageLayout) -> VkImageCreateInfoBuilder<'a> {
        self.s.initial_layout = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageCreateInfoBuilder<'a> {
    type Target = VkImageCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferViewCreateInfoBuilder<'a> {
    s: VkBufferViewCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBufferViewCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBufferViewCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkBufferViewCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsBufferViewCreateInfo>(mut self, next: &'a mut T) -> VkBufferViewCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkBufferViewCreateFlags) -> VkBufferViewCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn buffer(mut self, value: VkBuffer) -> VkBufferViewCreateInfoBuilder<'a> {
        self.s.buffer = value;
        self
    }

    pub fn format(mut self, value: VkFormat) -> VkBufferViewCreateInfoBuilder<'a> {
        self.s.format = value;
        self
    }

    pub fn offset(mut self, value: VkDeviceSize) -> VkBufferViewCreateInfoBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn range(mut self, value: VkDeviceSize) -> VkBufferViewCreateInfoBuilder<'a> {
        self.s.range = value;
        self
    }
}

impl<'a> core::ops::Deref for VkBufferViewCreateInfoBuilder<'a> {
    type Target = VkBufferViewCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBufferViewCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBufferCreateInfoBuilder<'a> {
    s: VkBufferCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBufferCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBufferCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkBufferCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsBufferCreateInfo>(mut self, next: &'a mut T) -> VkBufferCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkBufferCreateFlags) -> VkBufferCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkBufferCreateInfoBuilder<'a> {
        self.s.size = value;
        self
    }

    pub fn usage(mut self, value: VkBufferUsageFlags) -> VkBufferCreateInfoBuilder<'a> {
        self.s.usage = value;
        self
    }

    pub fn sharing_mode(mut self, value: VkSharingMode) -> VkBufferCreateInfoBuilder<'a> {
        self.s.sharing_mode = value;
        self
    }

    pub fn queue_family_index_count(mut self, value: u32) -> VkBufferCreateInfoBuilder<'a> {
        self.s.queue_family_index_count = value;
        self
    }

    pub fn p_queue_family_indices(mut self, values: &'a [u32]) -> VkBufferCreateInfoBuilder<'a> {
        self.s.queue_family_index_count = values.len() as _;
        self.s.p_queue_family_indices = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkBufferCreateInfoBuilder<'a> {
    type Target = VkBufferCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBufferCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkQueryPoolCreateInfoBuilder<'a> {
    s: VkQueryPoolCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkQueryPoolCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkQueryPoolCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkQueryPoolCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsQueryPoolCreateInfo>(mut self, next: &'a mut T) -> VkQueryPoolCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkQueryPoolCreateFlags) -> VkQueryPoolCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn query_type(mut self, value: VkQueryType) -> VkQueryPoolCreateInfoBuilder<'a> {
        self.s.query_type = value;
        self
    }

    pub fn query_count(mut self, value: u32) -> VkQueryPoolCreateInfoBuilder<'a> {
        self.s.query_count = value;
        self
    }

    pub fn pipeline_statistics(mut self, value: VkQueryPipelineStatisticFlags) -> VkQueryPoolCreateInfoBuilder<'a> {
        self.s.pipeline_statistics = value;
        self
    }
}

impl<'a> core::ops::Deref for VkQueryPoolCreateInfoBuilder<'a> {
    type Target = VkQueryPoolCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkQueryPoolCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkEventCreateInfoBuilder<'a> {
    s: VkEventCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkEventCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkEventCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkEventCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsEventCreateInfo>(mut self, next: &'a mut T) -> VkEventCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkEventCreateFlags) -> VkEventCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkEventCreateInfoBuilder<'a> {
    type Target = VkEventCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkEventCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSemaphoreCreateInfoBuilder<'a> {
    s: VkSemaphoreCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSemaphoreCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSemaphoreCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkSemaphoreCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsSemaphoreCreateInfo>(mut self, next: &'a mut T) -> VkSemaphoreCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkSemaphoreCreateFlags) -> VkSemaphoreCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSemaphoreCreateInfoBuilder<'a> {
    type Target = VkSemaphoreCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSemaphoreCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkFenceCreateInfoBuilder<'a> {
    s: VkFenceCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkFenceCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkFenceCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkFenceCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsFenceCreateInfo>(mut self, next: &'a mut T) -> VkFenceCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkFenceCreateFlags) -> VkFenceCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkFenceCreateInfoBuilder<'a> {
    type Target = VkFenceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkFenceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkBindSparseInfoBuilder<'a> {
    s: VkBindSparseInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkBindSparseInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkBindSparseInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkBindSparseInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsBindSparseInfo>(mut self, next: &'a mut T) -> VkBindSparseInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn wait_semaphore_count(mut self, value: u32) -> VkBindSparseInfoBuilder<'a> {
        self.s.wait_semaphore_count = value;
        self
    }

    pub fn p_wait_semaphores(mut self, values: &'a [VkSemaphore]) -> VkBindSparseInfoBuilder<'a> {
        self.s.wait_semaphore_count = values.len() as _;
        self.s.p_wait_semaphores = values.as_ptr();
        self
    }

    pub fn buffer_bind_count(mut self, value: u32) -> VkBindSparseInfoBuilder<'a> {
        self.s.buffer_bind_count = value;
        self
    }

    pub fn p_buffer_binds(mut self, values: &'a [VkSparseBufferMemoryBindInfo]) -> VkBindSparseInfoBuilder<'a> {
        self.s.buffer_bind_count = values.len() as _;
        self.s.p_buffer_binds = values.as_ptr();
        self
    }

    pub fn image_opaque_bind_count(mut self, value: u32) -> VkBindSparseInfoBuilder<'a> {
        self.s.image_opaque_bind_count = value;
        self
    }

    pub fn p_image_opaque_binds(mut self, values: &'a [VkSparseImageOpaqueMemoryBindInfo]) -> VkBindSparseInfoBuilder<'a> {
        self.s.image_opaque_bind_count = values.len() as _;
        self.s.p_image_opaque_binds = values.as_ptr();
        self
    }

    pub fn image_bind_count(mut self, value: u32) -> VkBindSparseInfoBuilder<'a> {
        self.s.image_bind_count = value;
        self
    }

    pub fn p_image_binds(mut self, values: &'a [VkSparseImageMemoryBindInfo]) -> VkBindSparseInfoBuilder<'a> {
        self.s.image_bind_count = values.len() as _;
        self.s.p_image_binds = values.as_ptr();
        self
    }

    pub fn signal_semaphore_count(mut self, value: u32) -> VkBindSparseInfoBuilder<'a> {
        self.s.signal_semaphore_count = value;
        self
    }

    pub fn p_signal_semaphores(mut self, values: &'a [VkSemaphore]) -> VkBindSparseInfoBuilder<'a> {
        self.s.signal_semaphore_count = values.len() as _;
        self.s.p_signal_semaphores = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkBindSparseInfoBuilder<'a> {
    type Target = VkBindSparseInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkBindSparseInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageMemoryBindInfoBuilder<'a> {
    s: VkSparseImageMemoryBindInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseImageMemoryBindInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageMemoryBindInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn image(mut self, value: VkImage) -> VkSparseImageMemoryBindInfoBuilder<'a> {
        self.s.image = value;
        self
    }

    pub fn bind_count(mut self, value: u32) -> VkSparseImageMemoryBindInfoBuilder<'a> {
        self.s.bind_count = value;
        self
    }

    pub fn p_binds(mut self, values: &'a [VkSparseImageMemoryBind]) -> VkSparseImageMemoryBindInfoBuilder<'a> {
        self.s.bind_count = values.len() as _;
        self.s.p_binds = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkSparseImageMemoryBindInfoBuilder<'a> {
    type Target = VkSparseImageMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseImageMemoryBindInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageMemoryBindBuilder<'a> {
    s: VkSparseImageMemoryBind,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseImageMemoryBindBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageMemoryBind::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn subresource(mut self, value: VkImageSubresource) -> VkSparseImageMemoryBindBuilder<'a> {
        self.s.subresource = value;
        self
    }

    pub fn offset(mut self, value: VkOffset3D) -> VkSparseImageMemoryBindBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn extent(mut self, value: VkExtent3D) -> VkSparseImageMemoryBindBuilder<'a> {
        self.s.extent = value;
        self
    }

    pub fn memory(mut self, value: VkDeviceMemory) -> VkSparseImageMemoryBindBuilder<'a> {
        self.s.memory = value;
        self
    }

    pub fn memory_offset(mut self, value: VkDeviceSize) -> VkSparseImageMemoryBindBuilder<'a> {
        self.s.memory_offset = value;
        self
    }

    pub fn flags(mut self, value: VkSparseMemoryBindFlags) -> VkSparseImageMemoryBindBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSparseImageMemoryBindBuilder<'a> {
    type Target = VkSparseImageMemoryBind;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseImageMemoryBindBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
    s: VkSparseImageOpaqueMemoryBindInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageOpaqueMemoryBindInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn image(mut self, value: VkImage) -> VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
        self.s.image = value;
        self
    }

    pub fn bind_count(mut self, value: u32) -> VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
        self.s.bind_count = value;
        self
    }

    pub fn p_binds(mut self, values: &'a [VkSparseMemoryBind]) -> VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
        self.s.bind_count = values.len() as _;
        self.s.p_binds = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
    type Target = VkSparseImageOpaqueMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseImageOpaqueMemoryBindInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseMemoryBindBuilder<'a> {
    s: VkSparseMemoryBind,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseMemoryBindBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseMemoryBind::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn resource_offset(mut self, value: VkDeviceSize) -> VkSparseMemoryBindBuilder<'a> {
        self.s.resource_offset = value;
        self
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkSparseMemoryBindBuilder<'a> {
        self.s.size = value;
        self
    }

    pub fn memory(mut self, value: VkDeviceMemory) -> VkSparseMemoryBindBuilder<'a> {
        self.s.memory = value;
        self
    }

    pub fn memory_offset(mut self, value: VkDeviceSize) -> VkSparseMemoryBindBuilder<'a> {
        self.s.memory_offset = value;
        self
    }

    pub fn flags(mut self, value: VkSparseMemoryBindFlags) -> VkSparseMemoryBindBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSparseMemoryBindBuilder<'a> {
    type Target = VkSparseMemoryBind;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseMemoryBindBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseBufferMemoryBindInfoBuilder<'a> {
    s: VkSparseBufferMemoryBindInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseBufferMemoryBindInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseBufferMemoryBindInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn buffer(mut self, value: VkBuffer) -> VkSparseBufferMemoryBindInfoBuilder<'a> {
        self.s.buffer = value;
        self
    }

    pub fn bind_count(mut self, value: u32) -> VkSparseBufferMemoryBindInfoBuilder<'a> {
        self.s.bind_count = value;
        self
    }

    pub fn p_binds(mut self, values: &'a [VkSparseMemoryBind]) -> VkSparseBufferMemoryBindInfoBuilder<'a> {
        self.s.bind_count = values.len() as _;
        self.s.p_binds = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkSparseBufferMemoryBindInfoBuilder<'a> {
    type Target = VkSparseBufferMemoryBindInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseBufferMemoryBindInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageFormatPropertiesBuilder<'a> {
    s: VkSparseImageFormatProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseImageFormatPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageFormatProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn aspect_mask(mut self, value: VkImageAspectFlags) -> VkSparseImageFormatPropertiesBuilder<'a> {
        self.s.aspect_mask = value;
        self
    }

    pub fn image_granularity(mut self, value: VkExtent3D) -> VkSparseImageFormatPropertiesBuilder<'a> {
        self.s.image_granularity = value;
        self
    }

    pub fn flags(mut self, value: VkSparseImageFormatFlags) -> VkSparseImageFormatPropertiesBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSparseImageFormatPropertiesBuilder<'a> {
    type Target = VkSparseImageFormatProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSparseImageMemoryRequirementsBuilder<'a> {
    s: VkSparseImageMemoryRequirements,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSparseImageMemoryRequirementsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSparseImageMemoryRequirements::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn format_properties(mut self, value: VkSparseImageFormatProperties) -> VkSparseImageMemoryRequirementsBuilder<'a> {
        self.s.format_properties = value;
        self
    }

    pub fn image_mip_tail_first_lod(mut self, value: u32) -> VkSparseImageMemoryRequirementsBuilder<'a> {
        self.s.image_mip_tail_first_lod = value;
        self
    }

    pub fn image_mip_tail_size(mut self, value: VkDeviceSize) -> VkSparseImageMemoryRequirementsBuilder<'a> {
        self.s.image_mip_tail_size = value;
        self
    }

    pub fn image_mip_tail_offset(mut self, value: VkDeviceSize) -> VkSparseImageMemoryRequirementsBuilder<'a> {
        self.s.image_mip_tail_offset = value;
        self
    }

    pub fn image_mip_tail_stride(mut self, value: VkDeviceSize) -> VkSparseImageMemoryRequirementsBuilder<'a> {
        self.s.image_mip_tail_stride = value;
        self
    }
}

impl<'a> core::ops::Deref for VkSparseImageMemoryRequirementsBuilder<'a> {
    type Target = VkSparseImageMemoryRequirements;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSparseImageMemoryRequirementsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryRequirementsBuilder<'a> {
    s: VkMemoryRequirements,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkMemoryRequirementsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkMemoryRequirements::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkMemoryRequirementsBuilder<'a> {
        self.s.size = value;
        self
    }

    pub fn alignment(mut self, value: VkDeviceSize) -> VkMemoryRequirementsBuilder<'a> {
        self.s.alignment = value;
        self
    }

    pub fn memory_type_bits(mut self, value: u32) -> VkMemoryRequirementsBuilder<'a> {
        self.s.memory_type_bits = value;
        self
    }
}

impl<'a> core::ops::Deref for VkMemoryRequirementsBuilder<'a> {
    type Target = VkMemoryRequirements;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkMemoryRequirementsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMappedMemoryRangeBuilder<'a> {
    s: VkMappedMemoryRange,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkMappedMemoryRangeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkMappedMemoryRange::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkMappedMemoryRangeBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsMappedMemoryRange>(mut self, next: &'a mut T) -> VkMappedMemoryRangeBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn memory(mut self, value: VkDeviceMemory) -> VkMappedMemoryRangeBuilder<'a> {
        self.s.memory = value;
        self
    }

    pub fn offset(mut self, value: VkDeviceSize) -> VkMappedMemoryRangeBuilder<'a> {
        self.s.offset = value;
        self
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkMappedMemoryRangeBuilder<'a> {
        self.s.size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkMappedMemoryRangeBuilder<'a> {
    type Target = VkMappedMemoryRange;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkMappedMemoryRangeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryAllocateInfoBuilder<'a> {
    s: VkMemoryAllocateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkMemoryAllocateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkMemoryAllocateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkMemoryAllocateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsMemoryAllocateInfo>(mut self, next: &'a mut T) -> VkMemoryAllocateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn allocation_size(mut self, value: VkDeviceSize) -> VkMemoryAllocateInfoBuilder<'a> {
        self.s.allocation_size = value;
        self
    }

    pub fn memory_type_index(mut self, value: u32) -> VkMemoryAllocateInfoBuilder<'a> {
        self.s.memory_type_index = value;
        self
    }
}

impl<'a> core::ops::Deref for VkMemoryAllocateInfoBuilder<'a> {
    type Target = VkMemoryAllocateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkMemoryAllocateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkSubmitInfoBuilder<'a> {
    s: VkSubmitInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkSubmitInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkSubmitInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkSubmitInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsSubmitInfo>(mut self, next: &'a mut T) -> VkSubmitInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn wait_semaphore_count(mut self, value: u32) -> VkSubmitInfoBuilder<'a> {
        self.s.wait_semaphore_count = value;
        self
    }

    pub fn p_wait_semaphores(mut self, values: &'a [VkSemaphore]) -> VkSubmitInfoBuilder<'a> {
        self.s.wait_semaphore_count = values.len() as _;
        self.s.p_wait_semaphores = values.as_ptr();
        self
    }

    pub fn p_wait_dst_stage_mask(mut self, values: &'a [VkPipelineStageFlags]) -> VkSubmitInfoBuilder<'a> {
        self.s.wait_semaphore_count = values.len() as _;
        self.s.p_wait_dst_stage_mask = values.as_ptr();
        self
    }

    pub fn command_buffer_count(mut self, value: u32) -> VkSubmitInfoBuilder<'a> {
        self.s.command_buffer_count = value;
        self
    }

    pub fn p_command_buffers(mut self, values: &'a [VkCommandBuffer]) -> VkSubmitInfoBuilder<'a> {
        self.s.command_buffer_count = values.len() as _;
        self.s.p_command_buffers = values.as_ptr();
        self
    }

    pub fn signal_semaphore_count(mut self, value: u32) -> VkSubmitInfoBuilder<'a> {
        self.s.signal_semaphore_count = value;
        self
    }

    pub fn p_signal_semaphores(mut self, values: &'a [VkSemaphore]) -> VkSubmitInfoBuilder<'a> {
        self.s.signal_semaphore_count = values.len() as _;
        self.s.p_signal_semaphores = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkSubmitInfoBuilder<'a> {
    type Target = VkSubmitInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkSubmitInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkLayerPropertiesBuilder<'a> {
    s: VkLayerProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkLayerPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkLayerProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn layer_name(mut self, value: [u8; 256]) -> VkLayerPropertiesBuilder<'a> {
        self.s.layer_name = value;
        self
    }

    pub fn spec_version(mut self, value: u32) -> VkLayerPropertiesBuilder<'a> {
        self.s.spec_version = value;
        self
    }

    pub fn implementation_version(mut self, value: u32) -> VkLayerPropertiesBuilder<'a> {
        self.s.implementation_version = value;
        self
    }

    pub fn description(mut self, value: [u8; 256]) -> VkLayerPropertiesBuilder<'a> {
        self.s.description = value;
        self
    }
}

impl<'a> core::ops::Deref for VkLayerPropertiesBuilder<'a> {
    type Target = VkLayerProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkLayerPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkExtensionPropertiesBuilder<'a> {
    s: VkExtensionProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkExtensionPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkExtensionProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn extension_name(mut self, value: [u8; 256]) -> VkExtensionPropertiesBuilder<'a> {
        self.s.extension_name = value;
        self
    }

    pub fn spec_version(mut self, value: u32) -> VkExtensionPropertiesBuilder<'a> {
        self.s.spec_version = value;
        self
    }
}

impl<'a> core::ops::Deref for VkExtensionPropertiesBuilder<'a> {
    type Target = VkExtensionProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkExtensionPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDeviceCreateInfoBuilder<'a> {
    s: VkDeviceCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDeviceCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDeviceCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDeviceCreateInfo>(mut self, next: &'a mut T) -> VkDeviceCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkDeviceCreateFlags) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn queue_create_info_count(mut self, value: u32) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.queue_create_info_count = value;
        self
    }

    pub fn p_queue_create_infos(mut self, values: &'a [VkDeviceQueueCreateInfo]) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.queue_create_info_count = values.len() as _;
        self.s.p_queue_create_infos = values.as_ptr();
        self
    }

    pub fn enabled_layer_count(mut self, value: u32) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.enabled_layer_count = value;
        self
    }

    pub fn pp_enabled_layer_names(mut self, values: &'a [*const u8]) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.enabled_layer_count = values.len() as _;
        self.s.pp_enabled_layer_names = values.as_ptr();
        self
    }

    pub fn enabled_extension_count(mut self, value: u32) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.enabled_extension_count = value;
        self
    }

    pub fn pp_enabled_extension_names(mut self, values: &'a [*const u8]) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.enabled_extension_count = values.len() as _;
        self.s.pp_enabled_extension_names = values.as_ptr();
        self
    }

    pub fn p_enabled_features(mut self, value: Option<&'a VkPhysicalDeviceFeatures>) -> VkDeviceCreateInfoBuilder<'a> {
        self.s.p_enabled_features = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }
}

impl<'a> core::ops::Deref for VkDeviceCreateInfoBuilder<'a> {
    type Target = VkDeviceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDeviceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceFeaturesBuilder<'a> {
    s: VkPhysicalDeviceFeatures,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPhysicalDeviceFeaturesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceFeatures::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn robust_buffer_access(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.robust_buffer_access = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn full_draw_index_uint_32(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.full_draw_index_uint_32 = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn image_cube_array(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.image_cube_array = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn independent_blend(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.independent_blend = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn geometry_shader(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.geometry_shader = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn tessellation_shader(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.tessellation_shader = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sample_rate_shading(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sample_rate_shading = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn dual_src_blend(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.dual_src_blend = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn logic_op(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.logic_op = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn multi_draw_indirect(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.multi_draw_indirect = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn draw_indirect_first_instance(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.draw_indirect_first_instance = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn depth_clamp(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.depth_clamp = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn depth_bias_clamp(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.depth_bias_clamp = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn fill_mode_non_solid(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.fill_mode_non_solid = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn depth_bounds(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.depth_bounds = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn wide_lines(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.wide_lines = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn large_points(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.large_points = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn alpha_to_one(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.alpha_to_one = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn multi_viewport(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.multi_viewport = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sampler_anisotropy(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sampler_anisotropy = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn texture_compression_etc_2(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.texture_compression_etc_2 = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn texture_compression_astc_ldr(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.texture_compression_astc_ldr = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn texture_compression_bc(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.texture_compression_bc = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn occlusion_query_precise(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.occlusion_query_precise = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn pipeline_statistics_query(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.pipeline_statistics_query = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn vertex_pipeline_stores_and_atomics(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.vertex_pipeline_stores_and_atomics = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn fragment_stores_and_atomics(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.fragment_stores_and_atomics = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_tessellation_and_geometry_point_size(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_tessellation_and_geometry_point_size = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_image_gather_extended(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_image_gather_extended = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_storage_image_extended_formats(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_storage_image_extended_formats = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_storage_image_multisample(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_storage_image_multisample = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_storage_image_read_without_format(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_storage_image_read_without_format = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_storage_image_write_without_format(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_storage_image_write_without_format = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_uniform_buffer_array_dynamic_indexing(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_uniform_buffer_array_dynamic_indexing = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_sampled_image_array_dynamic_indexing(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_sampled_image_array_dynamic_indexing = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_storage_buffer_array_dynamic_indexing(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_storage_buffer_array_dynamic_indexing = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_storage_image_array_dynamic_indexing(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_storage_image_array_dynamic_indexing = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_clip_distance(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_clip_distance = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_cull_distance(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_cull_distance = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_float_64(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_float_64 = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_int_64(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_int_64 = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_int_16(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_int_16 = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_resource_residency(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_resource_residency = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn shader_resource_min_lod(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.shader_resource_min_lod = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_binding(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_binding = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_buffer(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_buffer = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_image_2_d(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_image_2_d = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_image_3_d(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_image_3_d = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_2_samples(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_2_samples = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_4_samples(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_4_samples = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_8_samples(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_8_samples = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_16_samples(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_16_samples = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn sparse_residency_aliased(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.sparse_residency_aliased = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn variable_multisample_rate(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.variable_multisample_rate = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn inherited_queries(mut self, value: bool) -> VkPhysicalDeviceFeaturesBuilder<'a> {
        self.s.inherited_queries = if value { VK_TRUE } else { VK_FALSE };
        self
    }
}

impl<'a> core::ops::Deref for VkPhysicalDeviceFeaturesBuilder<'a> {
    type Target = VkPhysicalDeviceFeatures;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPhysicalDeviceFeaturesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkDeviceQueueCreateInfoBuilder<'a> {
    s: VkDeviceQueueCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkDeviceQueueCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkDeviceQueueCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkDeviceQueueCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsDeviceQueueCreateInfo>(mut self, next: &'a mut T) -> VkDeviceQueueCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkDeviceQueueCreateFlags) -> VkDeviceQueueCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn queue_family_index(mut self, value: u32) -> VkDeviceQueueCreateInfoBuilder<'a> {
        self.s.queue_family_index = value;
        self
    }

    pub fn queue_count(mut self, value: u32) -> VkDeviceQueueCreateInfoBuilder<'a> {
        self.s.queue_count = value;
        self
    }

    pub fn p_queue_priorities(mut self, values: &'a [f32]) -> VkDeviceQueueCreateInfoBuilder<'a> {
        self.s.queue_count = values.len() as _;
        self.s.p_queue_priorities = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkDeviceQueueCreateInfoBuilder<'a> {
    type Target = VkDeviceQueueCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkDeviceQueueCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
    s: VkPhysicalDeviceMemoryProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceMemoryProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn memory_type_count(mut self, value: u32) -> VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
        self.s.memory_type_count = value;
        self
    }

    pub fn memory_types(mut self, value: [VkMemoryType; 32]) -> VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
        self.s.memory_types = value;
        self
    }

    pub fn memory_heap_count(mut self, value: u32) -> VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
        self.s.memory_heap_count = value;
        self
    }

    pub fn memory_heaps(mut self, value: [VkMemoryHeap; 16]) -> VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
        self.s.memory_heaps = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
    type Target = VkPhysicalDeviceMemoryProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPhysicalDeviceMemoryPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryHeapBuilder<'a> {
    s: VkMemoryHeap,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkMemoryHeapBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkMemoryHeap::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn size(mut self, value: VkDeviceSize) -> VkMemoryHeapBuilder<'a> {
        self.s.size = value;
        self
    }

    pub fn flags(mut self, value: VkMemoryHeapFlags) -> VkMemoryHeapBuilder<'a> {
        self.s.flags = value;
        self
    }
}

impl<'a> core::ops::Deref for VkMemoryHeapBuilder<'a> {
    type Target = VkMemoryHeap;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkMemoryHeapBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkMemoryTypeBuilder<'a> {
    s: VkMemoryType,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkMemoryTypeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkMemoryType::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn property_flags(mut self, value: VkMemoryPropertyFlags) -> VkMemoryTypeBuilder<'a> {
        self.s.property_flags = value;
        self
    }

    pub fn heap_index(mut self, value: u32) -> VkMemoryTypeBuilder<'a> {
        self.s.heap_index = value;
        self
    }
}

impl<'a> core::ops::Deref for VkMemoryTypeBuilder<'a> {
    type Target = VkMemoryType;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkMemoryTypeBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkQueueFamilyPropertiesBuilder<'a> {
    s: VkQueueFamilyProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkQueueFamilyPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkQueueFamilyProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn queue_flags(mut self, value: VkQueueFlags) -> VkQueueFamilyPropertiesBuilder<'a> {
        self.s.queue_flags = value;
        self
    }

    pub fn queue_count(mut self, value: u32) -> VkQueueFamilyPropertiesBuilder<'a> {
        self.s.queue_count = value;
        self
    }

    pub fn timestamp_valid_bits(mut self, value: u32) -> VkQueueFamilyPropertiesBuilder<'a> {
        self.s.timestamp_valid_bits = value;
        self
    }

    pub fn min_image_transfer_granularity(mut self, value: VkExtent3D) -> VkQueueFamilyPropertiesBuilder<'a> {
        self.s.min_image_transfer_granularity = value;
        self
    }
}

impl<'a> core::ops::Deref for VkQueueFamilyPropertiesBuilder<'a> {
    type Target = VkQueueFamilyProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkQueueFamilyPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDevicePropertiesBuilder<'a> {
    s: VkPhysicalDeviceProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPhysicalDevicePropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn api_version(mut self, value: u32) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.api_version = value;
        self
    }

    pub fn driver_version(mut self, value: u32) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.driver_version = value;
        self
    }

    pub fn vendor_id(mut self, value: u32) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.vendor_id = value;
        self
    }

    pub fn device_id(mut self, value: u32) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.device_id = value;
        self
    }

    pub fn device_type(mut self, value: VkPhysicalDeviceType) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.device_type = value;
        self
    }

    pub fn device_name(mut self, value: [u8; 256]) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.device_name = value;
        self
    }

    pub fn pipeline_cache_uuid(mut self, value: [u8; 16]) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.pipeline_cache_uuid = value;
        self
    }

    pub fn limits(mut self, value: VkPhysicalDeviceLimits) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.limits = value;
        self
    }

    pub fn sparse_properties(mut self, value: VkPhysicalDeviceSparseProperties) -> VkPhysicalDevicePropertiesBuilder<'a> {
        self.s.sparse_properties = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPhysicalDevicePropertiesBuilder<'a> {
    type Target = VkPhysicalDeviceProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPhysicalDevicePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceSparsePropertiesBuilder<'a> {
    s: VkPhysicalDeviceSparseProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPhysicalDeviceSparsePropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceSparseProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn residency_standard_2_d_block_shape(mut self, value: bool) -> VkPhysicalDeviceSparsePropertiesBuilder<'a> {
        self.s.residency_standard_2_d_block_shape = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn residency_standard_2_d_multisample_block_shape(mut self, value: bool) -> VkPhysicalDeviceSparsePropertiesBuilder<'a> {
        self.s.residency_standard_2_d_multisample_block_shape = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn residency_standard_3_d_block_shape(mut self, value: bool) -> VkPhysicalDeviceSparsePropertiesBuilder<'a> {
        self.s.residency_standard_3_d_block_shape = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn residency_aligned_mip_size(mut self, value: bool) -> VkPhysicalDeviceSparsePropertiesBuilder<'a> {
        self.s.residency_aligned_mip_size = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn residency_non_resident_strict(mut self, value: bool) -> VkPhysicalDeviceSparsePropertiesBuilder<'a> {
        self.s.residency_non_resident_strict = if value { VK_TRUE } else { VK_FALSE };
        self
    }
}

impl<'a> core::ops::Deref for VkPhysicalDeviceSparsePropertiesBuilder<'a> {
    type Target = VkPhysicalDeviceSparseProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPhysicalDeviceSparsePropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkPhysicalDeviceLimitsBuilder<'a> {
    s: VkPhysicalDeviceLimits,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkPhysicalDeviceLimitsBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkPhysicalDeviceLimits::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn max_image_dimension_1_d(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_image_dimension_1_d = value;
        self
    }

    pub fn max_image_dimension_2_d(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_image_dimension_2_d = value;
        self
    }

    pub fn max_image_dimension_3_d(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_image_dimension_3_d = value;
        self
    }

    pub fn max_image_dimension_cube(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_image_dimension_cube = value;
        self
    }

    pub fn max_image_array_layers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_image_array_layers = value;
        self
    }

    pub fn max_texel_buffer_elements(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_texel_buffer_elements = value;
        self
    }

    pub fn max_uniform_buffer_range(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_uniform_buffer_range = value;
        self
    }

    pub fn max_storage_buffer_range(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_storage_buffer_range = value;
        self
    }

    pub fn max_push_constants_size(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_push_constants_size = value;
        self
    }

    pub fn max_memory_allocation_count(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_memory_allocation_count = value;
        self
    }

    pub fn max_sampler_allocation_count(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_sampler_allocation_count = value;
        self
    }

    pub fn buffer_image_granularity(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.buffer_image_granularity = value;
        self
    }

    pub fn sparse_address_space_size(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sparse_address_space_size = value;
        self
    }

    pub fn max_bound_descriptor_sets(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_bound_descriptor_sets = value;
        self
    }

    pub fn max_per_stage_descriptor_samplers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_descriptor_samplers = value;
        self
    }

    pub fn max_per_stage_descriptor_uniform_buffers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_descriptor_uniform_buffers = value;
        self
    }

    pub fn max_per_stage_descriptor_storage_buffers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_descriptor_storage_buffers = value;
        self
    }

    pub fn max_per_stage_descriptor_sampled_images(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_descriptor_sampled_images = value;
        self
    }

    pub fn max_per_stage_descriptor_storage_images(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_descriptor_storage_images = value;
        self
    }

    pub fn max_per_stage_descriptor_input_attachments(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_descriptor_input_attachments = value;
        self
    }

    pub fn max_per_stage_resources(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_per_stage_resources = value;
        self
    }

    pub fn max_descriptor_set_samplers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_samplers = value;
        self
    }

    pub fn max_descriptor_set_uniform_buffers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_uniform_buffers = value;
        self
    }

    pub fn max_descriptor_set_uniform_buffers_dynamic(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_uniform_buffers_dynamic = value;
        self
    }

    pub fn max_descriptor_set_storage_buffers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_storage_buffers = value;
        self
    }

    pub fn max_descriptor_set_storage_buffers_dynamic(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_storage_buffers_dynamic = value;
        self
    }

    pub fn max_descriptor_set_sampled_images(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_sampled_images = value;
        self
    }

    pub fn max_descriptor_set_storage_images(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_storage_images = value;
        self
    }

    pub fn max_descriptor_set_input_attachments(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_descriptor_set_input_attachments = value;
        self
    }

    pub fn max_vertex_input_attributes(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_vertex_input_attributes = value;
        self
    }

    pub fn max_vertex_input_bindings(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_vertex_input_bindings = value;
        self
    }

    pub fn max_vertex_input_attribute_offset(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_vertex_input_attribute_offset = value;
        self
    }

    pub fn max_vertex_input_binding_stride(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_vertex_input_binding_stride = value;
        self
    }

    pub fn max_vertex_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_vertex_output_components = value;
        self
    }

    pub fn max_tessellation_generation_level(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_generation_level = value;
        self
    }

    pub fn max_tessellation_patch_size(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_patch_size = value;
        self
    }

    pub fn max_tessellation_control_per_vertex_input_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_control_per_vertex_input_components = value;
        self
    }

    pub fn max_tessellation_control_per_vertex_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_control_per_vertex_output_components = value;
        self
    }

    pub fn max_tessellation_control_per_patch_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_control_per_patch_output_components = value;
        self
    }

    pub fn max_tessellation_control_total_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_control_total_output_components = value;
        self
    }

    pub fn max_tessellation_evaluation_input_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_evaluation_input_components = value;
        self
    }

    pub fn max_tessellation_evaluation_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_tessellation_evaluation_output_components = value;
        self
    }

    pub fn max_geometry_shader_invocations(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_geometry_shader_invocations = value;
        self
    }

    pub fn max_geometry_input_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_geometry_input_components = value;
        self
    }

    pub fn max_geometry_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_geometry_output_components = value;
        self
    }

    pub fn max_geometry_output_vertices(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_geometry_output_vertices = value;
        self
    }

    pub fn max_geometry_total_output_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_geometry_total_output_components = value;
        self
    }

    pub fn max_fragment_input_components(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_fragment_input_components = value;
        self
    }

    pub fn max_fragment_output_attachments(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_fragment_output_attachments = value;
        self
    }

    pub fn max_fragment_dual_src_attachments(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_fragment_dual_src_attachments = value;
        self
    }

    pub fn max_fragment_combined_output_resources(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_fragment_combined_output_resources = value;
        self
    }

    pub fn max_compute_shared_memory_size(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_compute_shared_memory_size = value;
        self
    }

    pub fn max_compute_work_group_count(mut self, value: [u32; 3]) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_compute_work_group_count = value;
        self
    }

    pub fn max_compute_work_group_invocations(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_compute_work_group_invocations = value;
        self
    }

    pub fn max_compute_work_group_size(mut self, value: [u32; 3]) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_compute_work_group_size = value;
        self
    }

    pub fn sub_pixel_precision_bits(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sub_pixel_precision_bits = value;
        self
    }

    pub fn sub_texel_precision_bits(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sub_texel_precision_bits = value;
        self
    }

    pub fn mipmap_precision_bits(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.mipmap_precision_bits = value;
        self
    }

    pub fn max_draw_indexed_index_value(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_draw_indexed_index_value = value;
        self
    }

    pub fn max_draw_indirect_count(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_draw_indirect_count = value;
        self
    }

    pub fn max_sampler_lod_bias(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_sampler_lod_bias = value;
        self
    }

    pub fn max_sampler_anisotropy(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_sampler_anisotropy = value;
        self
    }

    pub fn max_viewports(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_viewports = value;
        self
    }

    pub fn max_viewport_dimensions(mut self, value: [u32; 2]) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_viewport_dimensions = value;
        self
    }

    pub fn viewport_bounds_range(mut self, value: [f32; 2]) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.viewport_bounds_range = value;
        self
    }

    pub fn viewport_sub_pixel_bits(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.viewport_sub_pixel_bits = value;
        self
    }

    pub fn min_memory_map_alignment(mut self, value: usize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_memory_map_alignment = value;
        self
    }

    pub fn min_texel_buffer_offset_alignment(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_texel_buffer_offset_alignment = value;
        self
    }

    pub fn min_uniform_buffer_offset_alignment(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_uniform_buffer_offset_alignment = value;
        self
    }

    pub fn min_storage_buffer_offset_alignment(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_storage_buffer_offset_alignment = value;
        self
    }

    pub fn min_texel_offset(mut self, value: i32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_texel_offset = value;
        self
    }

    pub fn max_texel_offset(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_texel_offset = value;
        self
    }

    pub fn min_texel_gather_offset(mut self, value: i32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_texel_gather_offset = value;
        self
    }

    pub fn max_texel_gather_offset(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_texel_gather_offset = value;
        self
    }

    pub fn min_interpolation_offset(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.min_interpolation_offset = value;
        self
    }

    pub fn max_interpolation_offset(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_interpolation_offset = value;
        self
    }

    pub fn sub_pixel_interpolation_offset_bits(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sub_pixel_interpolation_offset_bits = value;
        self
    }

    pub fn max_framebuffer_width(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_framebuffer_width = value;
        self
    }

    pub fn max_framebuffer_height(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_framebuffer_height = value;
        self
    }

    pub fn max_framebuffer_layers(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_framebuffer_layers = value;
        self
    }

    pub fn framebuffer_color_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.framebuffer_color_sample_counts = value;
        self
    }

    pub fn framebuffer_depth_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.framebuffer_depth_sample_counts = value;
        self
    }

    pub fn framebuffer_stencil_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.framebuffer_stencil_sample_counts = value;
        self
    }

    pub fn framebuffer_no_attachments_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.framebuffer_no_attachments_sample_counts = value;
        self
    }

    pub fn max_color_attachments(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_color_attachments = value;
        self
    }

    pub fn sampled_image_color_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sampled_image_color_sample_counts = value;
        self
    }

    pub fn sampled_image_integer_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sampled_image_integer_sample_counts = value;
        self
    }

    pub fn sampled_image_depth_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sampled_image_depth_sample_counts = value;
        self
    }

    pub fn sampled_image_stencil_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.sampled_image_stencil_sample_counts = value;
        self
    }

    pub fn storage_image_sample_counts(mut self, value: VkSampleCountFlags) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.storage_image_sample_counts = value;
        self
    }

    pub fn max_sample_mask_words(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_sample_mask_words = value;
        self
    }

    pub fn timestamp_compute_and_graphics(mut self, value: bool) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.timestamp_compute_and_graphics = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn timestamp_period(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.timestamp_period = value;
        self
    }

    pub fn max_clip_distances(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_clip_distances = value;
        self
    }

    pub fn max_cull_distances(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_cull_distances = value;
        self
    }

    pub fn max_combined_clip_and_cull_distances(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.max_combined_clip_and_cull_distances = value;
        self
    }

    pub fn discrete_queue_priorities(mut self, value: u32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.discrete_queue_priorities = value;
        self
    }

    pub fn point_size_range(mut self, value: [f32; 2]) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.point_size_range = value;
        self
    }

    pub fn line_width_range(mut self, value: [f32; 2]) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.line_width_range = value;
        self
    }

    pub fn point_size_granularity(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.point_size_granularity = value;
        self
    }

    pub fn line_width_granularity(mut self, value: f32) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.line_width_granularity = value;
        self
    }

    pub fn strict_lines(mut self, value: bool) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.strict_lines = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn standard_sample_locations(mut self, value: bool) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.standard_sample_locations = if value { VK_TRUE } else { VK_FALSE };
        self
    }

    pub fn optimal_buffer_copy_offset_alignment(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.optimal_buffer_copy_offset_alignment = value;
        self
    }

    pub fn optimal_buffer_copy_row_pitch_alignment(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.optimal_buffer_copy_row_pitch_alignment = value;
        self
    }

    pub fn non_coherent_atom_size(mut self, value: VkDeviceSize) -> VkPhysicalDeviceLimitsBuilder<'a> {
        self.s.non_coherent_atom_size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkPhysicalDeviceLimitsBuilder<'a> {
    type Target = VkPhysicalDeviceLimits;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkPhysicalDeviceLimitsBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkImageFormatPropertiesBuilder<'a> {
    s: VkImageFormatProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkImageFormatPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkImageFormatProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn max_extent(mut self, value: VkExtent3D) -> VkImageFormatPropertiesBuilder<'a> {
        self.s.max_extent = value;
        self
    }

    pub fn max_mip_levels(mut self, value: u32) -> VkImageFormatPropertiesBuilder<'a> {
        self.s.max_mip_levels = value;
        self
    }

    pub fn max_array_layers(mut self, value: u32) -> VkImageFormatPropertiesBuilder<'a> {
        self.s.max_array_layers = value;
        self
    }

    pub fn sample_counts(mut self, value: VkSampleCountFlags) -> VkImageFormatPropertiesBuilder<'a> {
        self.s.sample_counts = value;
        self
    }

    pub fn max_resource_size(mut self, value: VkDeviceSize) -> VkImageFormatPropertiesBuilder<'a> {
        self.s.max_resource_size = value;
        self
    }
}

impl<'a> core::ops::Deref for VkImageFormatPropertiesBuilder<'a> {
    type Target = VkImageFormatProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkImageFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkFormatPropertiesBuilder<'a> {
    s: VkFormatProperties,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkFormatPropertiesBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkFormatProperties::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn linear_tiling_features(mut self, value: VkFormatFeatureFlags) -> VkFormatPropertiesBuilder<'a> {
        self.s.linear_tiling_features = value;
        self
    }

    pub fn optimal_tiling_features(mut self, value: VkFormatFeatureFlags) -> VkFormatPropertiesBuilder<'a> {
        self.s.optimal_tiling_features = value;
        self
    }

    pub fn buffer_features(mut self, value: VkFormatFeatureFlags) -> VkFormatPropertiesBuilder<'a> {
        self.s.buffer_features = value;
        self
    }
}

impl<'a> core::ops::Deref for VkFormatPropertiesBuilder<'a> {
    type Target = VkFormatProperties;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkFormatPropertiesBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkInstanceCreateInfoBuilder<'a> {
    s: VkInstanceCreateInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkInstanceCreateInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkInstanceCreateInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsInstanceCreateInfo>(mut self, next: &'a mut T) -> VkInstanceCreateInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn flags(mut self, value: VkInstanceCreateFlags) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.flags = value;
        self
    }

    pub fn p_application_info(mut self, value: Option<&'a VkApplicationInfo>) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.p_application_info = match value {
            Some(r) => r,
            None => core::ptr::null(),
        };
        self
    }

    pub fn enabled_layer_count(mut self, value: u32) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.enabled_layer_count = value;
        self
    }

    pub fn pp_enabled_layer_names(mut self, values: &'a [*const u8]) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.enabled_layer_count = values.len() as _;
        self.s.pp_enabled_layer_names = values.as_ptr();
        self
    }

    pub fn enabled_extension_count(mut self, value: u32) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.enabled_extension_count = value;
        self
    }

    pub fn pp_enabled_extension_names(mut self, values: &'a [*const u8]) -> VkInstanceCreateInfoBuilder<'a> {
        self.s.enabled_extension_count = values.len() as _;
        self.s.pp_enabled_extension_names = values.as_ptr();
        self
    }
}

impl<'a> core::ops::Deref for VkInstanceCreateInfoBuilder<'a> {
    type Target = VkInstanceCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkInstanceCreateInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

pub struct VkApplicationInfoBuilder<'a> {
    s: VkApplicationInfo,
    _p: core::marker::PhantomData<&'a ()>,
}

impl<'a> VkApplicationInfoBuilder<'a> {
    pub fn new() -> Self {
        Self {
            s: VkApplicationInfo::default(),
            _p: core::marker::PhantomData,
        }
    }

    pub fn s_type(mut self, value: VkStructureType) -> VkApplicationInfoBuilder<'a> {
        self.s.s_type = value;
        self
    }

    pub fn push_next<T: ExtendsApplicationInfo>(mut self, next: &'a mut T) -> VkApplicationInfoBuilder<'a> {
        unsafe {
            let last = get_last_base_out_struct_chain(next as *mut T as *mut VkBaseOutStructure);
            (*last).p_next = self.s.p_next as _;
            self.s.p_next = core::mem::transmute(next);
        }
        self
    }

    pub fn p_application_name(mut self, values: &'a [u8]) -> VkApplicationInfoBuilder<'a> {
        
        self.s.p_application_name = values.as_ptr();
        self
    }

    pub fn application_version(mut self, value: u32) -> VkApplicationInfoBuilder<'a> {
        self.s.application_version = value;
        self
    }

    pub fn p_engine_name(mut self, values: &'a [u8]) -> VkApplicationInfoBuilder<'a> {
        
        self.s.p_engine_name = values.as_ptr();
        self
    }

    pub fn engine_version(mut self, value: u32) -> VkApplicationInfoBuilder<'a> {
        self.s.engine_version = value;
        self
    }

    pub fn api_version(mut self, value: u32) -> VkApplicationInfoBuilder<'a> {
        self.s.api_version = value;
        self
    }
}

impl<'a> core::ops::Deref for VkApplicationInfoBuilder<'a> {
    type Target = VkApplicationInfo;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

impl<'a> core::ops::DerefMut for VkApplicationInfoBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

