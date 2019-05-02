use core::borrow::Borrow;
use core::cell::Cell;
use core::convert::AsRef;
use core::marker::{PhantomData, Send, Sync, Unsize};
use core::ops::{Deref, CoerceUnsized};
use core::ptr::{drop_in_place, NonNull, write};
use crate::alloc::{Allocator, alloc_one, GlobalAllocator};

struct SharedInner<T: ?Sized>
{
    pub strong_count: Cell<usize>,
    pub value: T,
}

impl<T: ?Sized> SharedInner<T>
{
    #[inline]
    pub fn inc_strong(&self)
    {
        self.strong_count.set(self.strong_count.get() + 1);
    }

    #[inline]
    pub fn dec_strong(&self)
    {
        self.strong_count.set(self.strong_count.get() - 1);
    }

    #[inline]
    pub fn strong_count(&self) -> usize
    {
        self.strong_count.get()
    }
}

pub struct Shared<T: ?Sized, A: Allocator>
{
    ptr: NonNull<SharedInner<T>>,
    alloc: A,
    _phantom: PhantomData<T>,
}

impl<T, A> !Send for Shared<T, A>
where
    T: ?Sized,
    A: Allocator,
{}

impl<T, A> !Sync for Shared<T, A>
where
    T: ?Sized,
    A: Allocator,
{}

impl<T, A> Shared<T, A>
where
    A: Allocator
{
    pub fn new_with(value: T, mut alloc: A) -> Self
    {
        let mut ptr = unsafe
        {
            alloc_one::<SharedInner<T>>(&mut alloc)
                .expect("Allocation error")
        };

        unsafe
        {
            write(ptr.as_mut(), SharedInner
            {
                strong_count: Cell::new(1),
                value: value,
            });
        }

        return Self
        {
            ptr,
            alloc,
            _phantom: PhantomData,
        };
    }
}

impl<T> Shared<T, GlobalAllocator>
{
    pub fn new(value: T) -> Self
    {
        Self::new_with(value, GlobalAllocator)
    }
}

impl<T, A> Clone for Shared<T, A>
where
    T: ?Sized,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self
    {
        let inner = unsafe { self.ptr.as_ref() };
        inner.inc_strong();

        Self
        {
            ptr: self.ptr,
            alloc: self.alloc.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, A> Drop for Shared<T, A>
where
    T: ?Sized,
    A: Allocator,
{
    fn drop(&mut self)
    {
        let inner = unsafe { self.ptr.as_ref() };
        inner.dec_strong();

        if inner.strong_count() == 0
        {
            unsafe
            {
                drop_in_place(self.ptr.as_mut());
                self.alloc.dealloc_aligned(self.ptr.cast().as_ptr());
            }
        }
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Shared<U, A>> for Shared<T, A> {}

impl<T: ?Sized, A: Allocator> Deref for Shared<T, A>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        unsafe
        {
            &self.ptr.as_ref().value
        }
    }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for Shared<T, A>
{
    #[inline]
    fn as_ref(&self) -> &T
    {
        self
    }
}

impl<T: ?Sized, A: Allocator> Borrow<T> for Shared<T, A>
{
    #[inline]
    fn borrow(&self) -> &T
    {
        self
    }
}
