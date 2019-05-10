use super::{Allocator, Layout};

use core::{ffi::c_void, ptr::NonNull};

static mut GLOBAL_ALLOCATOR: Option<&'static mut Allocator> = None;

pub unsafe fn set_global_allocator(alloc: &'static mut Allocator)
{
    if GLOBAL_ALLOCATOR.is_some()
    {
        panic!("Cannot set the global allocator twice");
    }

    GLOBAL_ALLOCATOR = Some(alloc);
}

pub struct GlobalAllocator;

impl Default for GlobalAllocator
{
    #[inline]
    fn default() -> GlobalAllocator
    {
        GlobalAllocator
    }
}

impl Clone for GlobalAllocator
{
    #[inline]
    fn clone(&self) -> Self
    {
        GlobalAllocator
    }
}

impl Allocator for GlobalAllocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<c_void>>
    {
        GLOBAL_ALLOCATOR
            .as_mut()
            .expect("No global allocator defined")
            .alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        GLOBAL_ALLOCATOR
            .as_mut()
            .expect("No global allocator defined")
            .dealloc(ptr)
    }
}
