mod layout;
mod allocator;
mod win32_heap;
mod global;

pub use layout::Layout;
pub use allocator::{Allocator, alloc_one, alloc_array};
pub use win32_heap::Win32HeapAllocator;
pub use global::{GlobalAllocator, set_global_allocator};
