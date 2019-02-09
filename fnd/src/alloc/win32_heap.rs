use core::ffi::c_void;
use super::{Allocator, Layout};

type HANDLE = *mut c_void;
type BOOL = i32;

pub struct Win32HeapAllocator
{
    heap: HANDLE,
}

#[link(name = "kernel32")]
extern "system"
{
    fn GetProcessHeap() -> HANDLE;
    fn HeapAlloc(heap: HANDLE, flags: u32, byte: usize) -> *mut c_void;
    fn HeapFree(heap: HANDLE, flags: u32, mem: *mut c_void) -> BOOL;
}

impl Default for Win32HeapAllocator
{
    fn default() -> Self
    {
        Self
        {
            heap: unsafe { GetProcessHeap() },
        }
    }
}

impl Allocator for &Win32HeapAllocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<*mut c_void>
    {
        let ptr = HeapAlloc(self.heap, 0, layout.size);

        if ptr.is_null()
        {
            None
        }
        else
        {
            Some(ptr)
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        HeapFree(self.heap, 0, ptr);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn allocator()
    {
        unsafe
        {
            let alloc = Win32HeapAllocator::default();
            let mut alloc_ref = &alloc;
            let ptr = alloc_ref.alloc(Layout::new(128));
            assert!(ptr.is_some());
            alloc_ref.dealloc(ptr.unwrap());
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
            Self{ alloc }
        }

        fn do_alloc(&mut self, size: usize)
        {
            unsafe
            {
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
