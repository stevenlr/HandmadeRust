#![no_std]

use core::ops::Range;

use fnd::bitflags;

pub mod capabilities;
mod command_buffer;
mod command_pool;
mod device;
mod instance;
mod queue;
mod surface;
mod swapchain;

pub use command_buffer::*;
pub use command_pool::*;
pub use device::*;
pub use instance::*;
pub use queue::*;
pub use surface::*;
pub use swapchain::*;

pub trait Backend: Sized
{
    type Error: core::fmt::Debug;
    type Instance: Instance<Self>;
    type Surface: Surface<Self>;
    type PhysicalDevice;
    type QueueFamilyGroup: QueueFamilyGroup;
    type InnerQueue: InnerQueue<Self>;
    type Device: Device<Self>;
    type Semaphore: Copy;
    type Fence: Copy;
    type Swapchain: Swapchain<Self>;
    type InnerCommandPool: InnerCommandPool<Self>;
    type InnerCommandBuffer: InnerCommandBuffer<Self>;
    type Image;
}

#[derive(Copy, Clone)]
pub enum Format
{
    Rgb8Unorm,
    Rgb8Snorm,
    Rgb8Srgb,
    Bgr8Unorm,
    Bgr8Snorm,
    Bgr8Srgb,
    Rgba8Unorm,
    Rgba8Snorm,
    Rgba8Srgb,
    Bgra8Unorm,
    Bgra8Snorm,
    Bgra8Srgb,
}

bitflags! {
    pub enum AccessMask: u32 {
        INDIRECT_COMMAND_READ = 0x00000001,
        INDEX_READ = 0x00000002,
        VERTEX_ATTRIBUTE_READ = 0x00000004,
        UNIFORM_READ = 0x00000008,
        INPUT_ATTACHMENT_READ = 0x00000010,
        SHADER_READ = 0x00000020,
        SHADER_WRITE = 0x00000040,
        COLOR_ATTACHMENT_READ = 0x00000080,
        COLOR_ATTACHMENT_WRITE = 0x00000100,
        DEPTH_STENCIL_ATTACHMENT_READ = 0x00000200,
        DEPTH_STENCIL_ATTACHMENT_WRITE = 0x00000400,
        TRANSFER_READ = 0x00000800,
        TRANSFER_WRITE = 0x00001000,
        HOST_READ = 0x00002000,
        HOST_WRITE = 0x00004000,
        MEMORY_READ = 0x00008000,
        MEMORY_WRITE = 0x00010000,
    }
}

#[derive(Copy, Clone)]
pub enum ImageLayout
{
    Undefined,
    General,
    ColorAttachmentOptimal,
    DepthStencilAttachmentOptimal,
    DepthStencilReadOnlyOptimal,
    ShaderReadOnlyOptimal,
    TransferSrcOptimal,
    TransferDstOptimal,
    Preinitialized,
    PresentSrc,
}

bitflags! {
    pub enum ImageAspectMask: u32 {
        COLOR = 0x00000001,
        DEPTH = 0x00000002,
        STENCIL = 0x00000004,
    }
}

pub struct ImageSubresourceRange
{
    pub aspects: ImageAspectMask,
    pub mip_levels: Range<u32>,
    pub layers: Range<u32>,
}

// @Todo Add buffer barriers when we have buffers.
pub enum Barrier<'a, B: Backend>
{
    Global(Range<AccessMask>),
    Image
    {
        access: Range<AccessMask>,
        layout: Range<ImageLayout>,
        queues: Option<Range<u32>>,
        image: &'a B::Image,
        range: ImageSubresourceRange,
    },
}

bitflags! {
    pub enum PipelineStageMask: u32 {
        TOP_OF_PIPE = 0x00000001,
        DRAW_INDIRECT = 0x00000002,
        VERTEX_INPUT = 0x00000004,
        VERTEX_SHADER = 0x00000008,
        TESSELLATION_CONTROL_SHADER = 0x00000010,
        TESSELLATION_EVALUATION_SHADER = 0x00000020,
        GEOMETRY_SHADER = 0x00000040,
        FRAGMENT_SHADER = 0x00000080,
        EARLY_FRAGMENT_TESTS = 0x00000100,
        LATE_FRAGMENT_TESTS = 0x00000200,
        COLOR_ATTACHMENT_OUTPUT = 0x00000400,
        COMPUTE_SHADER = 0x00000800,
        TRANSFER = 0x00001000,
        BOTTOM_OF_PIPE = 0x00002000,
        HOST = 0x00004000,
        ALL_GRAPHICS = 0x00008000,
        ALL_COMMANDS = 0x00010000,
    }
}

pub enum ClearColor
{
    Float([f32; 4]),
    Int32([i32; 4]),
    UInt32([u32; 4]),
}
