mod allocator;
mod global;
mod layout;
mod win32_heap;

pub use allocator::{alloc_array, alloc_one, Allocator};
pub use global::{set_global_allocator, GlobalAllocator};
pub use layout::Layout;
pub use win32_heap::Win32HeapAllocator;
