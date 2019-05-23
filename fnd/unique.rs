use crate::alloc::{alloc_one, Allocator, GlobalAllocator};
use core::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    marker::{PhantomData, Unsize},
    ops::{CoerceUnsized, Deref, DerefMut},
    pin::Pin,
    ptr::{drop_in_place, write, NonNull},
};

pub struct Unq<T: ?Sized, A: Allocator = GlobalAllocator>
{
    ptr: NonNull<T>,
    alloc: A,
    _phantom: PhantomData<T>,
}

impl<T, A: Allocator> Unq<T, A>
{
    pub fn new_with(value: T, mut alloc: A) -> Self
    {
        let mut ptr = unsafe { alloc_one::<T>(&mut alloc).expect("Allocation error") };

        unsafe {
            write(ptr.as_mut(), value);
        }

        Self {
            ptr,
            alloc,
            _phantom: PhantomData {},
        }
    }

    pub fn pin_with(value: T, alloc: A) -> Pin<Self>
    {
        unsafe { Pin::new_unchecked(Self::new_with(value, alloc)) }
    }
}

impl<T: ?Sized, A: Allocator> Unq<T, A>
{
    pub fn leak<'a>(unq: Unq<T, A>) -> &'a mut T
    where
        A: 'a,
    {
        let reference = unsafe { &mut *unq.ptr.as_ptr() };
        core::mem::forget(unq);
        return reference;
    }
}

impl<T> Unq<T, GlobalAllocator>
{
    pub fn new(value: T) -> Self
    {
        Self::new_with(value, GlobalAllocator)
    }

    pub fn pin(value: T) -> Pin<Self>
    {
        Self::pin_with(value, GlobalAllocator)
    }
}

impl<T: ?Sized, A: Allocator> Deref for Unq<T, A>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized, A: Allocator> DerefMut for Unq<T, A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for Unq<T, A>
{
    #[inline]
    fn as_ref(&self) -> &T
    {
        self
    }
}

impl<T: ?Sized, A: Allocator> AsMut<T> for Unq<T, A>
{
    #[inline]
    fn as_mut(&mut self) -> &mut T
    {
        self
    }
}

impl<T: ?Sized, A: Allocator> Borrow<T> for Unq<T, A>
{
    #[inline]
    fn borrow(&self) -> &T
    {
        self
    }
}

impl<T: ?Sized, A: Allocator> BorrowMut<T> for Unq<T, A>
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T
    {
        self
    }
}

impl<T: ?Sized, A: Allocator> Drop for Unq<T, A>
{
    fn drop(&mut self)
    {
        unsafe {
            drop_in_place(self.ptr.as_ptr());
            self.alloc.dealloc_aligned(self.ptr.cast().as_ptr());
        }
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Unq<U, A>> for Unq<T, A> {}

impl<T: ?Sized, A: Allocator> Unpin for Unq<T, A> {}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::{set_global_allocator, SystemAllocator};

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
        let alloc = SystemAllocator::default();
        let mut dropped = false;

        {
            let mut p = Unq::new_with(
                MyObject {
                    x: 1,
                    y: 2,
                    s: "hello",
                    dropped: &mut dropped,
                },
                &alloc,
            );

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
        Unq::new_with(MyObject2 { x }, alloc)
    }

    #[test]
    fn dst()
    {
        let alloc = SystemAllocator::default();
        let my_dst = create_dst(42, &alloc);
        assert!(my_dst.do_something() == 42);
    }

    fn create_closure<A: Allocator>(y: i32, alloc: A) -> Unq<Fn(i32) -> i32, A>
    {
        Unq::new_with(move |x| x + y, alloc)
    }

    #[test]
    fn closure()
    {
        let alloc = SystemAllocator::default();
        let closure = create_closure(5, &alloc);

        assert!(closure(5) == 10);
        assert!(closure(6) == 11);
    }

    #[test]
    fn leak()
    {
        let alloc = SystemAllocator::default();
        let int = Unq::new_with(45, &alloc);
        let int = Unq::leak(int);
        assert_eq!(45, *int);
    }

    fn is_static<T>(_: &'static T) {}

    #[test]
    fn with_global()
    {
        let alloc = SystemAllocator::default();
        let mut alloc_ref = &alloc;

        unsafe {
            set_global_allocator(core::mem::transmute(&mut alloc_ref as &mut Allocator));
        }

        let b = Unq::new(32);
        assert_eq!(32, *b);
        let b = Unq::leak(b);
        assert_eq!(32, *b);
        is_static(b);
    }
}
