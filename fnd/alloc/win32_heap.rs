use super::{Allocator, Layout};
use core::{ffi::c_void, ptr::NonNull};

use win32::kernel32;

#[derive(Clone, Copy, Default)]
pub struct Win32HeapAllocator;

impl Allocator for Win32HeapAllocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<c_void>>
    {
        let ptr = kernel32::HeapAlloc(kernel32::GetProcessHeap(), 0, layout.size);
        NonNull::new(ptr)
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        kernel32::HeapFree(kernel32::GetProcessHeap(), 0, ptr);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn allocator()
    {
        unsafe {
            let mut alloc = Win32HeapAllocator::default();
            let ptr = alloc.alloc(Layout::new(128));
            assert!(ptr.is_some());
            alloc.dealloc(ptr.unwrap().as_ptr());
        }
    }

    struct Container<A: Allocator>
    {
        alloc: A,
    }

    impl<A: Allocator> Container<A>
    {
        fn new(alloc: A) -> Self
        {
            Self { alloc }
        }

        fn do_alloc(&mut self, size: usize)
        {
            unsafe {
                self.alloc.alloc(Layout::new(size));
            }
        }
    }

    #[test]
    fn multiple_refs()
    {
        let alloc = Win32HeapAllocator::default();
        let mut c1 = Container::new(alloc);
        let mut c2 = Container::new(alloc);

        c1.do_alloc(128);
        c2.do_alloc(64);
        c1.do_alloc(128);
        c2.do_alloc(64);
    }
}
