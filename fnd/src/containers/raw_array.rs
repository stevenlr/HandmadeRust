use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr::null_mut;

use crate::alloc::{Layout, Allocator};

pub struct RawArray<T, A: Allocator>
{
    pub ptr: *mut T,
    pub capacity: usize,
    alloc: A,
    _phantom: PhantomData<T>,
}

impl<T, A: Allocator> RawArray<T, A>
{
    pub fn new(alloc: A) -> Self
    {
        Self
        {
            ptr: null_mut(),
            capacity: 0,
            alloc: alloc,
            _phantom: PhantomData{},
        }
    }

    pub fn reserve(&mut self, new_capacity: usize)
    {
        if new_capacity <= self.capacity
        {
            return;
        }

        let new_layout = Layout::from_type_array::<T>(new_capacity);

        let ptr = unsafe
        {
            self.alloc.alloc_aligned(new_layout)
                .expect("Allocation error")
                .as_ptr() as *mut T
        };

        if self.capacity > 0
        {
            unsafe
            {
                ptr.copy_from(self.ptr, self.capacity);
                self.alloc.dealloc_aligned(self.ptr as *mut c_void);
            }
        }

        self.ptr = ptr;
        self.capacity = new_capacity;
    }
}

impl<T, A: Allocator> Drop for RawArray<T, A>
{
    fn drop(&mut self)
    {
        if !self.ptr.is_null()
        {
            unsafe
            {
                self.alloc.dealloc_aligned(self.ptr as *mut c_void);
            }
        }
    }
}
