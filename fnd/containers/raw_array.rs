use core::{ffi::c_void, marker::PhantomData, mem::size_of, ptr::null_mut};

use crate::alloc::{alloc_array, Allocator};

pub(crate) struct RawArray<T, A: Allocator>
{
    pub(crate) ptr:      *mut T,
    pub(crate) capacity: usize,
    pub(crate) alloc:    A,
    _phantom:            PhantomData<T>,
}

impl<T, A: Allocator> RawArray<T, A>
{
    pub(crate) fn new(alloc: A) -> Self
    {
        let capacity = if size_of::<T>() == 0 { !0 } else { 0 };

        Self {
            ptr: null_mut(),
            capacity,
            alloc,
            _phantom: PhantomData {},
        }
    }

    pub(crate) fn reserve(&mut self, new_capacity: usize)
    {
        if new_capacity <= self.capacity
        {
            return;
        }

        let ptr = unsafe {
            alloc_array::<T>(&mut self.alloc, new_capacity)
                .expect("Allocation error")
                .as_ptr()
        };

        if self.capacity > 0
        {
            unsafe {
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
            unsafe {
                self.alloc.dealloc_aligned(self.ptr as *mut c_void);
            }
        }
    }
}
