use core::ffi::c_void;
use core::ptr::{write_unaligned, read_unaligned, NonNull};
use core::mem::size_of;
use super::Layout;

pub trait Allocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<c_void>>;
    unsafe fn dealloc(&mut self, ptr: *mut c_void);

    unsafe fn alloc_aligned(&mut self, layout: Layout) -> Option<NonNull<c_void>>
    {
        let actual_size = layout.size + layout.align - 1 + size_of::<usize>();
        let ptr = match self.alloc(Layout::new(actual_size))
        {
            Some(p) => p.as_ptr() as usize,
            None => return None,
        };

        let aligned_ptr = layout.align_up(ptr + size_of::<usize>());
        let actual_ptr_ptr = aligned_ptr - size_of::<usize>();

        write_unaligned(actual_ptr_ptr as *mut usize, ptr);

        Some(NonNull::new_unchecked(aligned_ptr as *mut c_void))
    }

    unsafe fn dealloc_aligned(&mut self, ptr: *mut c_void)
    {
        let aligned_ptr = ptr as usize;
        let actual_ptr_ptr = aligned_ptr - size_of::<usize>();
        let actual_ptr = read_unaligned(actual_ptr_ptr as *const usize);
        self.dealloc(actual_ptr as *mut c_void);
    }
}
