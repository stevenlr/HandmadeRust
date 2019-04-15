use crate::types::*;
use crate::instance::Instance;
use crate::commands::DeviceCommands;

#[derive(Clone)]
pub struct Device {
    handle: VkDevice,
    d: DeviceCommands,
}

impl Device {
    pub fn new(device: VkDevice, instance: &Instance) -> Self {
        let commands = DeviceCommands::load(|fn_name| {
            unsafe { instance.i.get_device_proc_addr(device, fn_name.as_ptr()) }
        });
        Self {
            handle: device,
            d: commands,
        }
    }

    pub fn queue_present_khr(&self,
        queue: VkQueue,
        p_present_info: &VkPresentInfoKHR) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.queue_present_khr(
                queue,
                p_present_info,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::SUBOPTIMAL_KHR => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn acquire_next_image_khr(&self,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: VkSemaphore,
        fence: VkFence) -> Result<(VkResult, u32), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.acquire_next_image_khr(
                self.handle,
                swapchain,
                timeout,
                semaphore,
                fence,
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            VkResult::TIMEOUT => Ok((ret, ret_value)),
            VkResult::NOT_READY => Ok((ret, ret_value)),
            VkResult::SUBOPTIMAL_KHR => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_swapchain_images_khr_count(&self,
        swapchain: VkSwapchainKHR) -> Result<(VkResult, usize), VkResult> {
        let mut p_swapchain_image_count = 0;
        let ret = unsafe {
            self.d.get_swapchain_images_khr(
                self.handle,
                swapchain,
                &mut p_swapchain_image_count,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_swapchain_image_count as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_swapchain_image_count as usize)),
            _ => Err(ret),
        };
    }

    pub fn get_swapchain_images_khr(&self,
        swapchain: VkSwapchainKHR,
        p_swapchain_images: &mut [VkImage]) -> Result<VkResult, VkResult> {
        let mut p_swapchain_image_count = p_swapchain_images.len() as _;
        let ret = unsafe {
            self.d.get_swapchain_images_khr(
                self.handle,
                swapchain,
                &mut p_swapchain_image_count,
                core::mem::transmute(p_swapchain_images.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_swapchain_khr(&self,
        swapchain: VkSwapchainKHR,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_swapchain_khr(
                self.handle,
                swapchain,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_swapchain_khr(&self,
        p_create_info: &VkSwapchainCreateInfoKHR,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSwapchainKHR), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_swapchain_khr(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn cmd_execute_commands(&self,
        command_buffer: VkCommandBuffer,
        p_command_buffers: &[VkCommandBuffer]) {
        let command_buffer_count = p_command_buffers.len() as _;
        let ret = unsafe {
            self.d.cmd_execute_commands(
                command_buffer,
                command_buffer_count,
                core::mem::transmute(p_command_buffers.as_ptr()),)
        };
    }

    pub fn cmd_end_render_pass(&self,
        command_buffer: VkCommandBuffer) {
        let ret = unsafe {
            self.d.cmd_end_render_pass(
                command_buffer,)
        };
    }

    pub fn cmd_next_subpass(&self,
        command_buffer: VkCommandBuffer,
        contents: VkSubpassContents) {
        let ret = unsafe {
            self.d.cmd_next_subpass(
                command_buffer,
                contents,)
        };
    }

    pub fn cmd_begin_render_pass(&self,
        command_buffer: VkCommandBuffer,
        p_render_pass_begin: &VkRenderPassBeginInfo,
        contents: VkSubpassContents) {
        let ret = unsafe {
            self.d.cmd_begin_render_pass(
                command_buffer,
                p_render_pass_begin,
                contents,)
        };
    }

    pub fn cmd_push_constants(&self,
        command_buffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stage_flags: VkShaderStageFlags,
        offset: u32,
        p_values: &[u8]) {
        let size = p_values.len() as _;
        let ret = unsafe {
            self.d.cmd_push_constants(
                command_buffer,
                layout,
                stage_flags,
                offset,
                size,
                core::mem::transmute(p_values.as_ptr()),)
        };
    }

    pub fn cmd_copy_query_pool_results(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: VkBuffer,
        dst_offset: VkDeviceSize,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags) {
        let ret = unsafe {
            self.d.cmd_copy_query_pool_results(
                command_buffer,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,)
        };
    }

    pub fn cmd_write_timestamp(&self,
        command_buffer: VkCommandBuffer,
        pipeline_stage: VkPipelineStageFlagBits,
        query_pool: VkQueryPool,
        query: u32) {
        let ret = unsafe {
            self.d.cmd_write_timestamp(
                command_buffer,
                pipeline_stage,
                query_pool,
                query,)
        };
    }

    pub fn cmd_reset_query_pool(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32) {
        let ret = unsafe {
            self.d.cmd_reset_query_pool(
                command_buffer,
                query_pool,
                first_query,
                query_count,)
        };
    }

    pub fn cmd_end_query(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        query: u32) {
        let ret = unsafe {
            self.d.cmd_end_query(
                command_buffer,
                query_pool,
                query,)
        };
    }

    pub fn cmd_begin_query(&self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        query: u32,
        flags: VkQueryControlFlags) {
        let ret = unsafe {
            self.d.cmd_begin_query(
                command_buffer,
                query_pool,
                query,
                flags,)
        };
    }

    pub fn cmd_pipeline_barrier(&self,
        command_buffer: VkCommandBuffer,
        src_stage_mask: VkPipelineStageFlags,
        dst_stage_mask: VkPipelineStageFlags,
        dependency_flags: VkDependencyFlags,
        p_memory_barriers: &[VkMemoryBarrier],
        p_buffer_memory_barriers: &[VkBufferMemoryBarrier],
        p_image_memory_barriers: &[VkImageMemoryBarrier]) {
        let memory_barrier_count = p_memory_barriers.len() as _;
        let buffer_memory_barrier_count = p_buffer_memory_barriers.len() as _;
        let image_memory_barrier_count = p_image_memory_barriers.len() as _;
        let ret = unsafe {
            self.d.cmd_pipeline_barrier(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barrier_count,
                core::mem::transmute(p_memory_barriers.as_ptr()),
                buffer_memory_barrier_count,
                core::mem::transmute(p_buffer_memory_barriers.as_ptr()),
                image_memory_barrier_count,
                core::mem::transmute(p_image_memory_barriers.as_ptr()),)
        };
    }

    pub fn cmd_wait_events(&self,
        command_buffer: VkCommandBuffer,
        p_events: &[VkEvent],
        src_stage_mask: VkPipelineStageFlags,
        dst_stage_mask: VkPipelineStageFlags,
        p_memory_barriers: &[VkMemoryBarrier],
        p_buffer_memory_barriers: &[VkBufferMemoryBarrier],
        p_image_memory_barriers: &[VkImageMemoryBarrier]) {
        let event_count = p_events.len() as _;
        let memory_barrier_count = p_memory_barriers.len() as _;
        let buffer_memory_barrier_count = p_buffer_memory_barriers.len() as _;
        let image_memory_barrier_count = p_image_memory_barriers.len() as _;
        let ret = unsafe {
            self.d.cmd_wait_events(
                command_buffer,
                event_count,
                core::mem::transmute(p_events.as_ptr()),
                src_stage_mask,
                dst_stage_mask,
                memory_barrier_count,
                core::mem::transmute(p_memory_barriers.as_ptr()),
                buffer_memory_barrier_count,
                core::mem::transmute(p_buffer_memory_barriers.as_ptr()),
                image_memory_barrier_count,
                core::mem::transmute(p_image_memory_barriers.as_ptr()),)
        };
    }

    pub fn cmd_reset_event(&self,
        command_buffer: VkCommandBuffer,
        event: VkEvent,
        stage_mask: VkPipelineStageFlags) {
        let ret = unsafe {
            self.d.cmd_reset_event(
                command_buffer,
                event,
                stage_mask,)
        };
    }

    pub fn cmd_set_event(&self,
        command_buffer: VkCommandBuffer,
        event: VkEvent,
        stage_mask: VkPipelineStageFlags) {
        let ret = unsafe {
            self.d.cmd_set_event(
                command_buffer,
                event,
                stage_mask,)
        };
    }

    pub fn cmd_resolve_image(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        p_regions: &[VkImageResolve]) {
        let region_count = p_regions.len() as _;
        let ret = unsafe {
            self.d.cmd_resolve_image(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                region_count,
                core::mem::transmute(p_regions.as_ptr()),)
        };
    }

    pub fn cmd_clear_attachments(&self,
        command_buffer: VkCommandBuffer,
        p_attachments: &[VkClearAttachment],
        p_rects: &[VkClearRect]) {
        let attachment_count = p_attachments.len() as _;
        let rect_count = p_rects.len() as _;
        let ret = unsafe {
            self.d.cmd_clear_attachments(
                command_buffer,
                attachment_count,
                core::mem::transmute(p_attachments.as_ptr()),
                rect_count,
                core::mem::transmute(p_rects.as_ptr()),)
        };
    }

    pub fn cmd_clear_depth_stencil_image(&self,
        command_buffer: VkCommandBuffer,
        image: VkImage,
        image_layout: VkImageLayout,
        p_depth_stencil: &VkClearDepthStencilValue,
        p_ranges: &[VkImageSubresourceRange]) {
        let range_count = p_ranges.len() as _;
        let ret = unsafe {
            self.d.cmd_clear_depth_stencil_image(
                command_buffer,
                image,
                image_layout,
                p_depth_stencil,
                range_count,
                core::mem::transmute(p_ranges.as_ptr()),)
        };
    }

    pub fn cmd_clear_color_image(&self,
        command_buffer: VkCommandBuffer,
        image: VkImage,
        image_layout: VkImageLayout,
        p_color: &VkClearColorValue,
        p_ranges: &[VkImageSubresourceRange]) {
        let range_count = p_ranges.len() as _;
        let ret = unsafe {
            self.d.cmd_clear_color_image(
                command_buffer,
                image,
                image_layout,
                p_color,
                range_count,
                core::mem::transmute(p_ranges.as_ptr()),)
        };
    }

    pub fn cmd_fill_buffer(&self,
        command_buffer: VkCommandBuffer,
        dst_buffer: VkBuffer,
        dst_offset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32) {
        let ret = unsafe {
            self.d.cmd_fill_buffer(
                command_buffer,
                dst_buffer,
                dst_offset,
                size,
                data,)
        };
    }

    pub fn cmd_update_buffer(&self,
        command_buffer: VkCommandBuffer,
        dst_buffer: VkBuffer,
        dst_offset: VkDeviceSize,
        p_data: &[u8]) {
        let data_size = p_data.len() as _;
        let ret = unsafe {
            self.d.cmd_update_buffer(
                command_buffer,
                dst_buffer,
                dst_offset,
                data_size,
                core::mem::transmute(p_data.as_ptr()),)
        };
    }

    pub fn cmd_copy_image_to_buffer(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_buffer: VkBuffer,
        p_regions: &[VkBufferImageCopy]) {
        let region_count = p_regions.len() as _;
        let ret = unsafe {
            self.d.cmd_copy_image_to_buffer(
                command_buffer,
                src_image,
                src_image_layout,
                dst_buffer,
                region_count,
                core::mem::transmute(p_regions.as_ptr()),)
        };
    }

    pub fn cmd_copy_buffer_to_image(&self,
        command_buffer: VkCommandBuffer,
        src_buffer: VkBuffer,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        p_regions: &[VkBufferImageCopy]) {
        let region_count = p_regions.len() as _;
        let ret = unsafe {
            self.d.cmd_copy_buffer_to_image(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                region_count,
                core::mem::transmute(p_regions.as_ptr()),)
        };
    }

    pub fn cmd_blit_image(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        p_regions: &[VkImageBlit],
        filter: VkFilter) {
        let region_count = p_regions.len() as _;
        let ret = unsafe {
            self.d.cmd_blit_image(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                region_count,
                core::mem::transmute(p_regions.as_ptr()),
                filter,)
        };
    }

    pub fn cmd_copy_image(&self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        p_regions: &[VkImageCopy]) {
        let region_count = p_regions.len() as _;
        let ret = unsafe {
            self.d.cmd_copy_image(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                region_count,
                core::mem::transmute(p_regions.as_ptr()),)
        };
    }

    pub fn cmd_copy_buffer(&self,
        command_buffer: VkCommandBuffer,
        src_buffer: VkBuffer,
        dst_buffer: VkBuffer,
        p_regions: &[VkBufferCopy]) {
        let region_count = p_regions.len() as _;
        let ret = unsafe {
            self.d.cmd_copy_buffer(
                command_buffer,
                src_buffer,
                dst_buffer,
                region_count,
                core::mem::transmute(p_regions.as_ptr()),)
        };
    }

    pub fn cmd_dispatch_indirect(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize) {
        let ret = unsafe {
            self.d.cmd_dispatch_indirect(
                command_buffer,
                buffer,
                offset,)
        };
    }

    pub fn cmd_dispatch(&self,
        command_buffer: VkCommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32) {
        let ret = unsafe {
            self.d.cmd_dispatch(
                command_buffer,
                group_count_x,
                group_count_y,
                group_count_z,)
        };
    }

    pub fn cmd_draw_indexed_indirect(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        draw_count: u32,
        stride: u32) {
        let ret = unsafe {
            self.d.cmd_draw_indexed_indirect(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,)
        };
    }

    pub fn cmd_draw_indirect(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        draw_count: u32,
        stride: u32) {
        let ret = unsafe {
            self.d.cmd_draw_indirect(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,)
        };
    }

    pub fn cmd_draw_indexed(&self,
        command_buffer: VkCommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32) {
        let ret = unsafe {
            self.d.cmd_draw_indexed(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,)
        };
    }

    pub fn cmd_draw(&self,
        command_buffer: VkCommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32) {
        let ret = unsafe {
            self.d.cmd_draw(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,)
        };
    }

    pub fn cmd_bind_vertex_buffers(&self,
        command_buffer: VkCommandBuffer,
        first_binding: u32,
        p_buffers: &[VkBuffer],
        p_offsets: &[VkDeviceSize]) {
        let binding_count = p_buffers.len() as _;
        assert!(binding_count as usize == p_offsets.len());
        let ret = unsafe {
            self.d.cmd_bind_vertex_buffers(
                command_buffer,
                first_binding,
                binding_count,
                core::mem::transmute(p_buffers.as_ptr()),
                core::mem::transmute(p_offsets.as_ptr()),)
        };
    }

    pub fn cmd_bind_index_buffer(&self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        index_type: VkIndexType) {
        let ret = unsafe {
            self.d.cmd_bind_index_buffer(
                command_buffer,
                buffer,
                offset,
                index_type,)
        };
    }

    pub fn cmd_bind_descriptor_sets(&self,
        command_buffer: VkCommandBuffer,
        pipeline_bind_point: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        first_set: u32,
        p_descriptor_sets: &[VkDescriptorSet],
        p_dynamic_offsets: &[u32]) {
        let descriptor_set_count = p_descriptor_sets.len() as _;
        let dynamic_offset_count = p_dynamic_offsets.len() as _;
        let ret = unsafe {
            self.d.cmd_bind_descriptor_sets(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_set_count,
                core::mem::transmute(p_descriptor_sets.as_ptr()),
                dynamic_offset_count,
                core::mem::transmute(p_dynamic_offsets.as_ptr()),)
        };
    }

    pub fn cmd_set_stencil_reference(&self,
        command_buffer: VkCommandBuffer,
        face_mask: VkStencilFaceFlags,
        reference: u32) {
        let ret = unsafe {
            self.d.cmd_set_stencil_reference(
                command_buffer,
                face_mask,
                reference,)
        };
    }

    pub fn cmd_set_stencil_write_mask(&self,
        command_buffer: VkCommandBuffer,
        face_mask: VkStencilFaceFlags,
        write_mask: u32) {
        let ret = unsafe {
            self.d.cmd_set_stencil_write_mask(
                command_buffer,
                face_mask,
                write_mask,)
        };
    }

    pub fn cmd_set_stencil_compare_mask(&self,
        command_buffer: VkCommandBuffer,
        face_mask: VkStencilFaceFlags,
        compare_mask: u32) {
        let ret = unsafe {
            self.d.cmd_set_stencil_compare_mask(
                command_buffer,
                face_mask,
                compare_mask,)
        };
    }

    pub fn cmd_set_depth_bounds(&self,
        command_buffer: VkCommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32) {
        let ret = unsafe {
            self.d.cmd_set_depth_bounds(
                command_buffer,
                min_depth_bounds,
                max_depth_bounds,)
        };
    }

    pub fn cmd_set_blend_constants(&self,
        command_buffer: VkCommandBuffer,
        blend_constants: [f32; 4]) {
        let ret = unsafe {
            self.d.cmd_set_blend_constants(
                command_buffer,
                blend_constants,)
        };
    }

    pub fn cmd_set_depth_bias(&self,
        command_buffer: VkCommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32) {
        let ret = unsafe {
            self.d.cmd_set_depth_bias(
                command_buffer,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,)
        };
    }

    pub fn cmd_set_line_width(&self,
        command_buffer: VkCommandBuffer,
        line_width: f32) {
        let ret = unsafe {
            self.d.cmd_set_line_width(
                command_buffer,
                line_width,)
        };
    }

    pub fn cmd_set_scissor(&self,
        command_buffer: VkCommandBuffer,
        first_scissor: u32,
        p_scissors: &[VkRect2D]) {
        let scissor_count = p_scissors.len() as _;
        let ret = unsafe {
            self.d.cmd_set_scissor(
                command_buffer,
                first_scissor,
                scissor_count,
                core::mem::transmute(p_scissors.as_ptr()),)
        };
    }

    pub fn cmd_set_viewport(&self,
        command_buffer: VkCommandBuffer,
        first_viewport: u32,
        p_viewports: &[VkViewport]) {
        let viewport_count = p_viewports.len() as _;
        let ret = unsafe {
            self.d.cmd_set_viewport(
                command_buffer,
                first_viewport,
                viewport_count,
                core::mem::transmute(p_viewports.as_ptr()),)
        };
    }

    pub fn cmd_bind_pipeline(&self,
        command_buffer: VkCommandBuffer,
        pipeline_bind_point: VkPipelineBindPoint,
        pipeline: VkPipeline) {
        let ret = unsafe {
            self.d.cmd_bind_pipeline(
                command_buffer,
                pipeline_bind_point,
                pipeline,)
        };
    }

    pub fn reset_command_buffer(&self,
        command_buffer: VkCommandBuffer,
        flags: VkCommandBufferResetFlags) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.reset_command_buffer(
                command_buffer,
                flags,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn end_command_buffer(&self,
        command_buffer: VkCommandBuffer) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.end_command_buffer(
                command_buffer,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn begin_command_buffer(&self,
        command_buffer: VkCommandBuffer,
        p_begin_info: &VkCommandBufferBeginInfo) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.begin_command_buffer(
                command_buffer,
                p_begin_info,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn free_command_buffers(&self,
        command_pool: VkCommandPool,
        p_command_buffers: &[VkCommandBuffer]) {
        let command_buffer_count = p_command_buffers.len() as _;
        let ret = unsafe {
            self.d.free_command_buffers(
                self.handle,
                command_pool,
                command_buffer_count,
                core::mem::transmute(p_command_buffers.as_ptr()),)
        };
    }

    pub fn allocate_command_buffers(&self,
        p_allocate_info: &VkCommandBufferAllocateInfo,
        p_command_buffers: &mut [VkCommandBuffer]) -> Result<VkResult, VkResult> {
        assert!(p_allocate_info.command_buffer_count as usize == p_command_buffers.len());
        let ret = unsafe {
            self.d.allocate_command_buffers(
                self.handle,
                p_allocate_info,
                core::mem::transmute(p_command_buffers.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn reset_command_pool(&self,
        command_pool: VkCommandPool,
        flags: VkCommandPoolResetFlags) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.reset_command_pool(
                self.handle,
                command_pool,
                flags,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_command_pool(&self,
        command_pool: VkCommandPool,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_command_pool(
                self.handle,
                command_pool,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_command_pool(&self,
        p_create_info: &VkCommandPoolCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkCommandPool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_command_pool(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_render_area_granularity(&self,
        render_pass: VkRenderPass) -> VkExtent2D {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_render_area_granularity(
                self.handle,
                render_pass,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn destroy_render_pass(&self,
        render_pass: VkRenderPass,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_render_pass(
                self.handle,
                render_pass,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_render_pass(&self,
        p_create_info: &VkRenderPassCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkRenderPass), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_render_pass(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_framebuffer(&self,
        framebuffer: VkFramebuffer,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_framebuffer(
                self.handle,
                framebuffer,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_framebuffer(&self,
        p_create_info: &VkFramebufferCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkFramebuffer), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_framebuffer(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn update_descriptor_sets(&self,
        p_descriptor_writes: &[VkWriteDescriptorSet],
        p_descriptor_copies: &[VkCopyDescriptorSet]) {
        let descriptor_write_count = p_descriptor_writes.len() as _;
        let descriptor_copy_count = p_descriptor_copies.len() as _;
        let ret = unsafe {
            self.d.update_descriptor_sets(
                self.handle,
                descriptor_write_count,
                core::mem::transmute(p_descriptor_writes.as_ptr()),
                descriptor_copy_count,
                core::mem::transmute(p_descriptor_copies.as_ptr()),)
        };
    }

    pub fn free_descriptor_sets(&self,
        descriptor_pool: VkDescriptorPool,
        p_descriptor_sets: &[VkDescriptorSet]) -> Result<VkResult, VkResult> {
        let descriptor_set_count = p_descriptor_sets.len() as _;
        let ret = unsafe {
            self.d.free_descriptor_sets(
                self.handle,
                descriptor_pool,
                descriptor_set_count,
                core::mem::transmute(p_descriptor_sets.as_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn allocate_descriptor_sets(&self,
        p_allocate_info: &VkDescriptorSetAllocateInfo,
        p_descriptor_sets: &mut [VkDescriptorSet]) -> Result<VkResult, VkResult> {
        assert!(p_allocate_info.descriptor_set_count as usize == p_descriptor_sets.len());
        let ret = unsafe {
            self.d.allocate_descriptor_sets(
                self.handle,
                p_allocate_info,
                core::mem::transmute(p_descriptor_sets.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn reset_descriptor_pool(&self,
        descriptor_pool: VkDescriptorPool,
        flags: VkDescriptorPoolResetFlags) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.reset_descriptor_pool(
                self.handle,
                descriptor_pool,
                flags,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_descriptor_pool(&self,
        descriptor_pool: VkDescriptorPool,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_descriptor_pool(
                self.handle,
                descriptor_pool,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_descriptor_pool(&self,
        p_create_info: &VkDescriptorPoolCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDescriptorPool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_descriptor_pool(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_descriptor_set_layout(&self,
        descriptor_set_layout: VkDescriptorSetLayout,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_descriptor_set_layout(
                self.handle,
                descriptor_set_layout,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_descriptor_set_layout(&self,
        p_create_info: &VkDescriptorSetLayoutCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDescriptorSetLayout), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_descriptor_set_layout(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_sampler(&self,
        sampler: VkSampler,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_sampler(
                self.handle,
                sampler,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_sampler(&self,
        p_create_info: &VkSamplerCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSampler), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_sampler(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_pipeline_layout(&self,
        pipeline_layout: VkPipelineLayout,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_pipeline_layout(
                self.handle,
                pipeline_layout,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_pipeline_layout(&self,
        p_create_info: &VkPipelineLayoutCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkPipelineLayout), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_pipeline_layout(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_pipeline(&self,
        pipeline: VkPipeline,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_pipeline(
                self.handle,
                pipeline,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_compute_pipelines(&self,
        pipeline_cache: VkPipelineCache,
        p_create_infos: &[VkComputePipelineCreateInfo],
        p_allocator: Option<&VkAllocationCallbacks>,
        p_pipelines: &mut [VkPipeline]) -> Result<VkResult, VkResult> {
        let create_info_count = p_create_infos.len() as _;
        assert!(create_info_count as usize == p_pipelines.len());
        let ret = unsafe {
            self.d.create_compute_pipelines(
                self.handle,
                pipeline_cache,
                create_info_count,
                core::mem::transmute(p_create_infos.as_ptr()),
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                core::mem::transmute(p_pipelines.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn create_graphics_pipelines(&self,
        pipeline_cache: VkPipelineCache,
        p_create_infos: &[VkGraphicsPipelineCreateInfo],
        p_allocator: Option<&VkAllocationCallbacks>,
        p_pipelines: &mut [VkPipeline]) -> Result<VkResult, VkResult> {
        let create_info_count = p_create_infos.len() as _;
        assert!(create_info_count as usize == p_pipelines.len());
        let ret = unsafe {
            self.d.create_graphics_pipelines(
                self.handle,
                pipeline_cache,
                create_info_count,
                core::mem::transmute(p_create_infos.as_ptr()),
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                core::mem::transmute(p_pipelines.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn merge_pipeline_caches(&self,
        dst_cache: VkPipelineCache,
        p_src_caches: &[VkPipelineCache]) -> Result<VkResult, VkResult> {
        let src_cache_count = p_src_caches.len() as _;
        let ret = unsafe {
            self.d.merge_pipeline_caches(
                self.handle,
                dst_cache,
                src_cache_count,
                core::mem::transmute(p_src_caches.as_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_pipeline_cache_data_count(&self,
        pipeline_cache: VkPipelineCache) -> Result<(VkResult, usize), VkResult> {
        let mut p_data_size = 0;
        let ret = unsafe {
            self.d.get_pipeline_cache_data(
                self.handle,
                pipeline_cache,
                &mut p_data_size,
                core::ptr::null_mut(),)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, p_data_size as usize)),
            VkResult::INCOMPLETE => Ok((ret, p_data_size as usize)),
            _ => Err(ret),
        };
    }

    pub fn get_pipeline_cache_data(&self,
        pipeline_cache: VkPipelineCache,
        p_data: &mut [u8]) -> Result<VkResult, VkResult> {
        let mut p_data_size = p_data.len() as _;
        let ret = unsafe {
            self.d.get_pipeline_cache_data(
                self.handle,
                pipeline_cache,
                &mut p_data_size,
                core::mem::transmute(p_data.as_mut_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::INCOMPLETE => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_pipeline_cache(&self,
        pipeline_cache: VkPipelineCache,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_pipeline_cache(
                self.handle,
                pipeline_cache,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_pipeline_cache(&self,
        p_create_info: &VkPipelineCacheCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkPipelineCache), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_pipeline_cache(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_shader_module(&self,
        shader_module: VkShaderModule,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_shader_module(
                self.handle,
                shader_module,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_shader_module(&self,
        p_create_info: &VkShaderModuleCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkShaderModule), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_shader_module(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_image_view(&self,
        image_view: VkImageView,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_image_view(
                self.handle,
                image_view,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_image_view(&self,
        p_create_info: &VkImageViewCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkImageView), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_image_view(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_image_subresource_layout(&self,
        image: VkImage,
        p_subresource: &VkImageSubresource) -> VkSubresourceLayout {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_image_subresource_layout(
                self.handle,
                image,
                p_subresource,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn destroy_image(&self,
        image: VkImage,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_image(
                self.handle,
                image,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_image(&self,
        p_create_info: &VkImageCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkImage), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_image(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_buffer_view(&self,
        buffer_view: VkBufferView,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_buffer_view(
                self.handle,
                buffer_view,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_buffer_view(&self,
        p_create_info: &VkBufferViewCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkBufferView), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_buffer_view(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_buffer(&self,
        buffer: VkBuffer,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_buffer(
                self.handle,
                buffer,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_buffer(&self,
        p_create_info: &VkBufferCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkBuffer), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_buffer(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn get_query_pool_results(&self,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        p_data: &mut [u8],
        stride: VkDeviceSize,
        flags: VkQueryResultFlags) -> Result<VkResult, VkResult> {
        let data_size = p_data.len() as _;
        let ret = unsafe {
            self.d.get_query_pool_results(
                self.handle,
                query_pool,
                first_query,
                query_count,
                data_size,
                core::mem::transmute(p_data.as_mut_ptr()),
                stride,
                flags,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::NOT_READY => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_query_pool(&self,
        query_pool: VkQueryPool,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_query_pool(
                self.handle,
                query_pool,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_query_pool(&self,
        p_create_info: &VkQueryPoolCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkQueryPool), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_query_pool(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn reset_event(&self,
        event: VkEvent) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.reset_event(
                self.handle,
                event,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn set_event(&self,
        event: VkEvent) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.set_event(
                self.handle,
                event,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_event_status(&self,
        event: VkEvent) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.get_event_status(
                self.handle,
                event,)
        };
        return match ret {
            VkResult::EVENT_SET => Ok(ret),
            VkResult::EVENT_RESET => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_event(&self,
        event: VkEvent,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_event(
                self.handle,
                event,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_event(&self,
        p_create_info: &VkEventCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkEvent), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_event(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn destroy_semaphore(&self,
        semaphore: VkSemaphore,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_semaphore(
                self.handle,
                semaphore,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_semaphore(&self,
        p_create_info: &VkSemaphoreCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkSemaphore), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_semaphore(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn wait_for_fences(&self,
        p_fences: &[VkFence],
        wait_all: bool,
        timeout: u64) -> Result<VkResult, VkResult> {
        let fence_count = p_fences.len() as _;
        let ret = unsafe {
            self.d.wait_for_fences(
                self.handle,
                fence_count,
                core::mem::transmute(p_fences.as_ptr()),
                if wait_all { VK_TRUE } else { VK_FALSE },
                timeout,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::TIMEOUT => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_fence_status(&self,
        fence: VkFence) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.get_fence_status(
                self.handle,
                fence,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            VkResult::NOT_READY => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn reset_fences(&self,
        p_fences: &[VkFence]) -> Result<VkResult, VkResult> {
        let fence_count = p_fences.len() as _;
        let ret = unsafe {
            self.d.reset_fences(
                self.handle,
                fence_count,
                core::mem::transmute(p_fences.as_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn destroy_fence(&self,
        fence: VkFence,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_fence(
                self.handle,
                fence,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn create_fence(&self,
        p_create_info: &VkFenceCreateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkFence), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.create_fence(
                self.handle,
                p_create_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn queue_bind_sparse(&self,
        queue: VkQueue,
        p_bind_info: &[VkBindSparseInfo],
        fence: VkFence) -> Result<VkResult, VkResult> {
        let bind_info_count = p_bind_info.len() as _;
        let ret = unsafe {
            self.d.queue_bind_sparse(
                queue,
                bind_info_count,
                core::mem::transmute(p_bind_info.as_ptr()),
                fence,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_image_sparse_memory_requirements_count(&self,
        image: VkImage) -> usize {
        let mut p_sparse_memory_requirement_count = 0;
        let ret = unsafe {
            self.d.get_image_sparse_memory_requirements(
                self.handle,
                image,
                &mut p_sparse_memory_requirement_count,
                core::ptr::null_mut(),)
        };
        return p_sparse_memory_requirement_count as usize;
    }

    pub fn get_image_sparse_memory_requirements(&self,
        image: VkImage,
        p_sparse_memory_requirements: &mut [VkSparseImageMemoryRequirements]) {
        let mut p_sparse_memory_requirement_count = p_sparse_memory_requirements.len() as _;
        let ret = unsafe {
            self.d.get_image_sparse_memory_requirements(
                self.handle,
                image,
                &mut p_sparse_memory_requirement_count,
                core::mem::transmute(p_sparse_memory_requirements.as_mut_ptr()),)
        };
    }

    pub fn get_image_memory_requirements(&self,
        image: VkImage) -> VkMemoryRequirements {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_image_memory_requirements(
                self.handle,
                image,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn get_buffer_memory_requirements(&self,
        buffer: VkBuffer) -> VkMemoryRequirements {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_buffer_memory_requirements(
                self.handle,
                buffer,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn bind_image_memory(&self,
        image: VkImage,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.bind_image_memory(
                self.handle,
                image,
                memory,
                memory_offset,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn bind_buffer_memory(&self,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memory_offset: VkDeviceSize) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.bind_buffer_memory(
                self.handle,
                buffer,
                memory,
                memory_offset,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_device_memory_commitment(&self,
        memory: VkDeviceMemory) -> VkDeviceSize {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_device_memory_commitment(
                self.handle,
                memory,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn invalidate_mapped_memory_ranges(&self,
        p_memory_ranges: &[VkMappedMemoryRange]) -> Result<VkResult, VkResult> {
        let memory_range_count = p_memory_ranges.len() as _;
        let ret = unsafe {
            self.d.invalidate_mapped_memory_ranges(
                self.handle,
                memory_range_count,
                core::mem::transmute(p_memory_ranges.as_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn flush_mapped_memory_ranges(&self,
        p_memory_ranges: &[VkMappedMemoryRange]) -> Result<VkResult, VkResult> {
        let memory_range_count = p_memory_ranges.len() as _;
        let ret = unsafe {
            self.d.flush_mapped_memory_ranges(
                self.handle,
                memory_range_count,
                core::mem::transmute(p_memory_ranges.as_ptr()),)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn unmap_memory(&self,
        memory: VkDeviceMemory) {
        let ret = unsafe {
            self.d.unmap_memory(
                self.handle,
                memory,)
        };
    }

    pub fn map_memory(&self,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags) -> Result<(VkResult, *mut core::ffi::c_void), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.map_memory(
                self.handle,
                memory,
                offset,
                size,
                flags,
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn free_memory(&self,
        memory: VkDeviceMemory,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.free_memory(
                self.handle,
                memory,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }

    pub fn allocate_memory(&self,
        p_allocate_info: &VkMemoryAllocateInfo,
        p_allocator: Option<&VkAllocationCallbacks>) -> Result<(VkResult, VkDeviceMemory), VkResult> {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.allocate_memory(
                self.handle,
                p_allocate_info,
                match p_allocator { Some(r) => r, None => core::ptr::null() },
                &mut ret_value,)
        };
        return match ret {
            VkResult::SUCCESS => Ok((ret, ret_value)),
            _ => Err(ret),
        };
    }

    pub fn device_wait_idle(&self) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.device_wait_idle(
                self.handle,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn queue_wait_idle(&self,
        queue: VkQueue) -> Result<VkResult, VkResult> {
        let ret = unsafe {
            self.d.queue_wait_idle(
                queue,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn queue_submit(&self,
        queue: VkQueue,
        p_submits: &[VkSubmitInfo],
        fence: VkFence) -> Result<VkResult, VkResult> {
        let submit_count = p_submits.len() as _;
        let ret = unsafe {
            self.d.queue_submit(
                queue,
                submit_count,
                core::mem::transmute(p_submits.as_ptr()),
                fence,)
        };
        return match ret {
            VkResult::SUCCESS => Ok(ret),
            _ => Err(ret),
        };
    }

    pub fn get_device_queue(&self,
        queue_family_index: u32,
        queue_index: u32) -> VkQueue {
        let mut ret_value = unsafe { core::mem::uninitialized() };
        let ret = unsafe {
            self.d.get_device_queue(
                self.handle,
                queue_family_index,
                queue_index,
                &mut ret_value,)
        };
        return ret_value;
    }

    pub fn destroy_device(&self,
        p_allocator: Option<&VkAllocationCallbacks>) {
        let ret = unsafe {
            self.d.destroy_device(
                self.handle,
                match p_allocator { Some(r) => r, None => core::ptr::null() },)
        };
    }
}
