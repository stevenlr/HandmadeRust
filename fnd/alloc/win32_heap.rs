use super::{Allocator, Layout};
use core::ffi::c_void;
use core::ptr::NonNull;

use win32::{kernel32, HANDLE};

pub struct Win32HeapAllocator
{
    heap: HANDLE,
}

impl Default for Win32HeapAllocator
{
    fn default() -> Self
    {
        Self {
            heap: unsafe { kernel32::GetProcessHeap() },
        }
    }
}

impl Allocator for &Win32HeapAllocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<c_void>>
    {
        let ptr = kernel32::HeapAlloc(self.heap, 0, layout.size);

        if ptr.is_null()
        {
            None
        }
        else
        {
            Some(NonNull::new_unchecked(ptr))
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        kernel32::HeapFree(self.heap, 0, ptr);
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
            let alloc = Win32HeapAllocator::default();
            let mut alloc_ref = &alloc;
            let ptr = alloc_ref.alloc(Layout::new(128));
            assert!(ptr.is_some());
            alloc_ref.dealloc(ptr.unwrap().as_ptr());
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
        let mut c1 = Container::new(&alloc);
        let mut c2 = Container::new(&alloc);

        c1.do_alloc(128);
        c2.do_alloc(64);
        c1.do_alloc(128);
        c2.do_alloc(64);
    }
}
