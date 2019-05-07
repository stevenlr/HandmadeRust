use core::iter::FromIterator;
use core::mem::needs_drop;
use core::ops::{Deref, DerefMut};
use core::ptr::drop_in_place;
use core::slice;

use crate::alloc::{Allocator, GlobalAllocator, Layout};
use crate::containers::RawArray;

pub struct Array<T, A: Allocator = GlobalAllocator>
{
    size: usize,
    buffer: RawArray<T, A>,
}

impl<T, A: Allocator> Array<T, A>
{
    pub fn new_with(alloc: A) -> Self
    {
        Self {
            buffer: RawArray::new(alloc),
            size: 0,
        }
    }

    pub fn resize_with<F>(&mut self, new_size: usize, f: F)
    where
        F: Fn() -> T,
    {
        if new_size < self.size && needs_drop::<T>()
        {
            for i in new_size..self.size
            {
                unsafe {
                    drop_in_place(self.buffer.ptr.offset(i as isize));
                }
            }
        }
        else if new_size > self.size
        {
            if new_size > self.buffer.capacity
            {
                self.reserve(new_size);
            }

            for i in self.size..new_size
            {
                unsafe {
                    self.buffer.ptr.offset(i as isize).write(f());
                }
            }
        }

        self.size = new_size;
    }

    pub fn resize(&mut self, new_size: usize, value: T)
    where
        T: Clone,
    {
        self.resize_with(new_size, || value.clone());
    }

    pub fn reserve(&mut self, new_capacity: usize)
    {
        self.buffer.reserve(new_capacity);
    }

    fn grow_auto(&mut self)
    {
        let single_layout = Layout::from_type::<T>();

        let old_capacity_bytes = self.buffer.capacity * single_layout.size;
        assert!(old_capacity_bytes <= (core::usize::MAX / 4));

        let new_capacity = if self.buffer.capacity == 0
        {
            1
        }
        else
        {
            self.buffer.capacity * 2
        };

        self.reserve(new_capacity);
    }

    #[inline]
    pub fn len(&self) -> usize
    {
        self.size
    }

    pub fn push(&mut self, value: T)
    {
        if self.size == self.buffer.capacity
        {
            self.grow_auto();
        }

        unsafe {
            self.buffer.ptr.offset(self.size as isize).write(value);
        }

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T>
    {
        if self.size == 0
        {
            None
        }
        else
        {
            let value = unsafe { self.buffer.ptr.offset((self.size - 1) as isize).read() };

            self.size -= 1;
            Some(value)
        }
    }

    pub fn clear(&mut self)
    {
        if needs_drop::<T>()
        {
            unsafe {
                for i in 0..self.size
                {
                    drop_in_place(self.buffer.ptr.offset(i as isize));
                }
            }
        }

        self.size = 0;
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
        self.size == 0
    }
}

impl<T> Array<T, GlobalAllocator>
{
    pub fn new() -> Self
    {
        Self::new_with(GlobalAllocator)
    }
}

impl<T, A: Allocator> Drop for Array<T, A>
{
    fn drop(&mut self)
    {
        if !self.buffer.ptr.is_null()
        {
            self.clear();
        }
    }
}

impl<T, A: Allocator> Deref for Array<T, A>
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        unsafe { slice::from_raw_parts(self.buffer.ptr, self.size) }
    }
}

impl<T, A: Allocator> DerefMut for Array<T, A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe { slice::from_raw_parts_mut(self.buffer.ptr, self.size) }
    }
}

impl<T, A: Allocator> Extend<T> for Array<T, A>
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for e in iter
        {
            self.push(e);
        }
    }
}

impl<'a, T: 'a, A: Allocator> Extend<&'a T> for Array<T, A>
where
    T: Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        for e in iter
        {
            self.push(*e);
        }
    }
}

impl<T, A: Allocator> FromIterator<T> for Array<T, A>
where
    A: Default,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut array = Array::new_with(A::default());
        array.extend(iter);
        return array;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::SystemAllocator;

    struct DropCheck<'a>
    {
        pub dropped: &'a mut bool,
    }

    impl<'a> DropCheck<'a>
    {
        fn new(b: &'a mut bool) -> Self
        {
            Self { dropped: b }
        }
    }

    impl<'a> Drop for DropCheck<'a>
    {
        fn drop(&mut self)
        {
            *self.dropped = true;
        }
    }

    #[test]
    fn push_pop()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);

        assert!(a.len() == 5);
        assert!(a.pop() == Some(5));
        assert!(a.pop() == Some(4));
        assert!(a.pop() == Some(3));
        assert!(a.len() == 2);

        a.push(3);
        a.push(4);
        a.push(5);

        assert!(a.len() == 5);
        assert!(a.pop() == Some(5));
        assert!(a.pop() == Some(4));
        assert!(a.pop() == Some(3));
        assert!(a.pop() == Some(2));
        assert!(a.pop() == Some(1));
        assert!(a.pop() == None);
        assert!(a.pop() == None);
        assert!(a.pop() == None);
        assert!(a.pop() == None);
        assert!(a.len() == 0);
    }

    #[test]
    fn drop()
    {
        let alloc = SystemAllocator::default();
        let mut dropped = false;

        {
            let mut a = Array::new_with(&alloc);
            a.push(DropCheck::new(&mut dropped));
        }

        assert!(dropped);
    }

    fn sum_slice(slice: &[i32]) -> i32
    {
        slice.iter().sum()
    }

    fn double_slice(slice: &mut [i32])
    {
        slice.iter_mut().for_each(|x| *x = *x * 2);
    }

    #[test]
    fn slice()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);

        assert!(sum_slice(&a) == 15);
        double_slice(&mut a);

        assert!(a[0] == 2);
        assert!(a[1] == 4);
        assert!(a[2] == 6);
        assert!(a[3] == 8);
        assert!(a[4] == 10);
    }

    #[test]
    fn subslice()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);

        assert!(sum_slice(&a[1..3]) == 5);
        assert!(sum_slice(&a[1..=3]) == 9);

        double_slice(&mut a[1..3]);

        assert!(a[0] == 1);
        assert!(a[1] == 4);
        assert!(a[2] == 6);
        assert!(a[3] == 4);
        assert!(a[4] == 5);

        double_slice(&mut a[1..=3]);

        assert!(a[0] == 1);
        assert!(a[1] == 8);
        assert!(a[2] == 12);
        assert!(a[3] == 8);
        assert!(a[4] == 5);
    }

    #[test]
    fn iter()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(1);
        a.push(2);
        a.push(3);
        a.push(4);
        a.push(5);

        for (i, value) in a.iter().enumerate()
        {
            assert!(i as i32 == value - 1);
        }

        for (i, value) in a.iter_mut().enumerate()
        {
            assert!(i as i32 == *value - 1);
        }
    }

    #[test]
    fn resize()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(1);
        a.resize(3, 7);
        a.push(2);
        a.push(3);

        assert!(a[0] == 1);
        assert!(a[1] == 7);
        assert!(a[2] == 7);
        assert!(a[3] == 2);
        assert!(a[4] == 3);
    }

    #[test]
    fn extend()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(1);
        a.extend([7, 8].iter());
        a.extend(&[2, 3]);

        assert!(a[0] == 1);
        assert!(a[1] == 7);
        assert!(a[2] == 8);
        assert!(a[3] == 2);
        assert!(a[4] == 3);
    }

    #[test]
    fn zst()
    {
        let alloc = SystemAllocator::default();
        let mut a = Array::new_with(&alloc);

        a.push(());
        a.push(());
        a.push(());
        assert!(a.len() == 3);

        assert!(a[1] == ());

        a.clear();
        assert!(a.len() == 0);
    }
}
