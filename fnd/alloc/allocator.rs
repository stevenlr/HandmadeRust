use super::Layout;
use core::{ffi::c_void, mem::size_of, ptr::NonNull};

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

        (actual_ptr_ptr as *mut usize).write_unaligned(ptr);

        Some(NonNull::new_unchecked(aligned_ptr as *mut c_void))
    }

    unsafe fn dealloc_aligned(&mut self, ptr: *mut c_void)
    {
        let aligned_ptr = ptr as usize;
        let actual_ptr_ptr = aligned_ptr - size_of::<usize>();
        let actual_ptr = (actual_ptr_ptr as *const usize).read_unaligned();
        self.dealloc(actual_ptr as *mut c_void);
    }
}

pub unsafe fn alloc_one<T>(alloc: &mut dyn Allocator) -> Option<NonNull<T>>
{
    alloc
        .alloc_aligned(Layout::from_type::<T>())
        .map(|ptr| ptr.cast::<T>())
}

pub unsafe fn alloc_array<T>(alloc: &mut dyn Allocator, size: usize) -> Option<NonNull<T>>
{
    alloc
        .alloc_aligned(Layout::from_type_array::<T>(size))
        .map(|ptr| ptr.cast::<T>())
}
