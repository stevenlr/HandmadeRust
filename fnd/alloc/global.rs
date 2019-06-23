use super::{Allocator, Layout, SystemAllocator};

use core::{cell::UnsafeCell, ffi::c_void, ptr::NonNull};

static mut GLOBAL_SYSTEM_ALLOCATOR: UnsafeCell<SystemAllocator> =
    UnsafeCell::new(SystemAllocator {});
static mut GLOBAL_ALLOCATOR: *mut dyn Allocator = unsafe { GLOBAL_SYSTEM_ALLOCATOR.get() };

pub unsafe fn set_global_allocator(alloc: &'static mut dyn Allocator)
{
    GLOBAL_ALLOCATOR = alloc;
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
        (*GLOBAL_ALLOCATOR).alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        (*GLOBAL_ALLOCATOR).dealloc(ptr)
    }
}
