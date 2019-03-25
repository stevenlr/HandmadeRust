use core::ffi::c_void;
use core::ptr::{NonNull, drop_in_place, write};
use core::mem::{replace, forget};
use core::marker::{Unsize, PhantomData};
use core::ops::{Deref, DerefMut, CoerceUnsized};
use crate::alloc::{Layout, Allocator};

pub struct Unq<T: ?Sized, A: Allocator>
{
    ptr: NonNull<T>,
    alloc: A,
    _phantom: PhantomData<T>,
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

        unsafe
        {
            write(ptr.as_mut(), value);
        }

        Self
        {
            ptr,
            alloc,
            _phantom: PhantomData{},
        }
    }
}

impl<T: ?Sized, A: Allocator> Deref for Unq<T, A>
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

impl<T: ?Sized, A: Allocator> DerefMut for Unq<T, A>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe
        {
            self.ptr.as_mut()
        }
    }
}

impl<T: ?Sized, A: Allocator> Drop for Unq<T, A>
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

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Unq<U, A>> for Unq<T, A> {}

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
    fn simple()
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

    struct MyObject2
    {
        x: i32,
    }

    trait MyTrait
    {
        fn do_something(&self) -> i32;
    }

    impl MyTrait for MyObject2
    {
        fn do_something(&self) -> i32
        {
            self.x
        }
    }

    fn create_dst<A: Allocator>(x: i32, alloc: A) -> Unq<dyn MyTrait, A>
    {
        Unq::new(MyObject2{ x }, alloc)
    }

    #[test]
    fn dst()
    {
        let alloc = Win32HeapAllocator::default();
        let my_dst = create_dst(42, &alloc);
        assert!(my_dst.do_something() == 42);
    }

    fn create_closure<A: Allocator>(y: i32, alloc: A) -> Unq<Fn(i32) -> i32, A>
    {
        Unq::new(move |x| x + y, alloc)
    }

    #[test]
    fn closure()
    {
        let alloc = Win32HeapAllocator::default();
        let closure = create_closure(5, &alloc);

        assert!(closure(5) == 10);
        assert!(closure(6) == 11);
    }
}
