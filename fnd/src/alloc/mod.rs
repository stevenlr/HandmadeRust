mod win32_heap;

pub use win32_heap::Win32HeapAllocator;
use core::ffi::c_void;
use core::mem::{size_of, align_of};
use core::ptr::{write_unaligned, read_unaligned};

pub struct Layout
{
    pub size: usize,
    pub align: usize,
}

impl Layout
{
    pub fn new(size: usize) -> Self
    {
        Self
        {
            size,
            align: 4,
        }
    }

    pub fn from_type<T>() -> Self
    {
        Self
        {
            size: size_of::<T>(),
            align: align_of::<T>(),
        }
    }

    pub fn align_up(&self, i: usize) -> usize
    {
        let p = i + self.align - 1;
        return p - (p % self.align);
    }
}

pub trait Allocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<*mut c_void>;
    unsafe fn dealloc(&mut self, ptr: *mut c_void);

    unsafe fn alloc_aligned(&mut self, layout: Layout) -> Option<*mut c_void>
    {
        let actual_size = layout.size + layout.align - 1 + size_of::<usize>();
        let ptr = match self.alloc(Layout::new(actual_size))
        {
            Some(p) => p as usize,
            None => return None,
        };

        let aligned_ptr = layout.align_up(ptr + size_of::<usize>());
        let actual_ptr_ptr = aligned_ptr - size_of::<usize>();

        write_unaligned(actual_ptr_ptr as *mut usize, ptr);

        Some(aligned_ptr as *mut c_void)
    }

    unsafe fn dealloc_aligned(&mut self, ptr: *mut c_void)
    {
        let aligned_ptr = ptr as usize;
        let actual_ptr_ptr = aligned_ptr - size_of::<usize>();
        let actual_ptr = read_unaligned(actual_ptr_ptr as *const usize);
        self.dealloc(actual_ptr as *mut c_void);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn layout_from_type()
    {
        let l1 = Layout::from_type::<i64>();
        assert!(l1.size == 8);
        assert!(l1.align == 8);
        
        let l2 = Layout::from_type::<u8>();
        assert!(l2.size == 1);
        assert!(l2.align == 1);
    }

    #[test]
    fn align_up()
    {
        let layout = Layout::from_type::<i32>();
        assert!(layout.align_up(0) == 0);
        assert!(layout.align_up(1) == 4);
        assert!(layout.align_up(2) == 4);
        assert!(layout.align_up(3) == 4);
        assert!(layout.align_up(4) == 4);
        assert!(layout.align_up(5) == 8);
        assert!(layout.align_up(6) == 8);
    }
}
