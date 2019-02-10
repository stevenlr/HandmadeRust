use core::ffi::c_void;
use core::ptr::{NonNull, drop_in_place};
use core::mem::{replace, forget};
use core::ops::{Deref, DerefMut};
use crate::alloc::{Layout, Allocator};

pub struct Unq<T, A: Allocator>
{
    ptr: NonNull<T>,
    alloc: A,
}

impl<T, A: Allocator> Unq<T, A>
{
    pub fn new(value: T, mut alloc: A) -> Self
    {
        let layout = Layout::from_type::<T>();
        let mut ptr = unsafe
        {
            alloc.alloc_aligned(layout)
                .expect("Allocation error")
                .cast::<T>()
        };

        forget(replace(unsafe{ ptr.as_mut() }, value));

        Self
        {
            ptr,
            alloc,
        }
    }
}

impl<T, A: Allocator> Deref for Unq<T, A>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        unsafe
        {
            self.ptr.as_ref()
        }
    }
}

impl<T, A: Allocator> DerefMut for Unq<T, A>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe
        {
            self.ptr.as_mut()
        }
    }
}

impl<T, A: Allocator> Drop for Unq<T, A>
{
    fn drop(&mut self)
    {
        unsafe
        {
            drop_in_place(self.ptr.as_ptr());
            self.alloc.dealloc_aligned(self.ptr.cast::<c_void>().as_ptr());
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::Win32HeapAllocator;

    struct MyObject<'a>
    {
        x: i32,
        y: i32,
        s: &'static str,
        dropped: &'a mut bool,
    }

    impl<'a> MyObject<'a>
    {
        fn add(&self) -> i32
        {
            self.x + self.y
        }

        fn set(&mut self, x: i32, y: i32)
        {
            self.x = x;
            self.y = y;
        }
    }

    impl<'a> Drop for MyObject<'a>
    {
        fn drop(&mut self)
        {
            *self.dropped = true;
        }
    }

    #[test]
    fn test()
    {
        let alloc = Win32HeapAllocator::default();
        let mut dropped = false;

        {
            let mut p = Unq::new(MyObject{ x: 1, y: 2, s: "hello", dropped: &mut dropped }, &alloc);

            assert!(p.x == 1);
            assert!(p.y == 2);
            assert!(p.s == "hello");

            p.x = 45;
            assert!(p.x == 45);
            assert!(p.y == 2);
            assert!(p.s == "hello");

            assert!(p.add() == 47);

            p.set(8, 9);
            assert!(p.x == 8);
            assert!(p.y == 9);
            assert!(p.s == "hello");

            assert!(*p.dropped == false);
        }

        assert!(dropped == true);
    }
}
