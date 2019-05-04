mod allocator;
mod global;
mod layout;

#[cfg(windows)]
mod win32_heap;

pub use allocator::{alloc_array, alloc_one, Allocator};
pub use global::{set_global_allocator, GlobalAllocator};
pub use layout::Layout;

#[cfg(windows)]
pub type SystemAllocator = win32_heap::Win32HeapAllocator;
