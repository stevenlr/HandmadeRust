use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::needs_drop;
use core::ops::{Index, IndexMut, Deref, DerefMut, Range, RangeInclusive};
use core::ptr::{drop_in_place, null_mut};
use core::slice;
use crate::alloc::{Layout, Allocator};

pub struct Array<T, A: Allocator>
{
    ptr: *mut T,
    capacity: usize,
    size: usize,
    alloc: A,
    _phantom: PhantomData<T>,
}

impl<T, A: Allocator> Array<T, A>
{
    pub fn new(alloc: A) -> Self
    {
        Self
        {
            ptr: null_mut(),
            capacity: 0,
            size: 0,
            alloc: alloc,
            _phantom: PhantomData{},
        }
    }

    pub fn resize(&mut self, new_size: usize, value: T)
        where T: Clone
    {
        if new_size <= self.size && needs_drop::<T>()
        {
            for i in new_size..self.size
            {
                unsafe
                {
                    drop_in_place(self.ptr.offset(i as isize));
                }
            }
        }
        else
        {
            if new_size > self.capacity
            {
                self.reserve(new_size);
            }

            for i in self.size..new_size
            {
                unsafe
                {
                    self.ptr.offset(i as isize).write(value.clone());
                }
            }
        }

        self.size = new_size;
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
                ptr.copy_from(self.ptr, self.size);
                self.alloc.dealloc_aligned(self.ptr as *mut c_void);
            }
        }

        self.ptr = ptr;
        self.capacity = new_capacity;
    }

    fn grow_auto(&mut self)
    {
        let single_layout = Layout::from_type::<T>();

        let old_capacity_bytes = self.capacity * single_layout.size;
        assert!(old_capacity_bytes <= (core::usize::MAX / 4));

        let new_capacity = if self.capacity == 0
        {
            1
        }
        else
        {
            self.capacity * 2
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
        if self.size == self.capacity
        {
            self.grow_auto();
        }

        unsafe
        {
            self.ptr.offset(self.size as isize).write(value);
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
            let value = unsafe
            {
                self.ptr.offset((self.size - 1) as isize).read()
            };

            self.size -= 1;
            Some(value)
        }
    }

    pub fn clear(&mut self)
    {
        if needs_drop::<T>()
        {
            unsafe
            {
                for i in 0..self.size
                {
                    drop_in_place(self.ptr.offset(i as isize));
                }
            }
        }

        self.size = 0;
    }
}

impl<T, A: Allocator> Index<usize> for Array<T, A>
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T
    {
        assert!(index < self.size);
        return unsafe
        {
            self.ptr.offset(index as isize).as_ref().unwrap()
        };
    }
}

impl<T, A: Allocator> IndexMut<usize> for Array<T, A>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T
    {
        assert!(index < self.size);
        return unsafe
        {
            self.ptr.offset(index as isize).as_mut().unwrap()
        };
    }
}

impl<T, A: Allocator> Index<Range<usize>> for Array<T, A>
{
    type Output = [T];

    #[inline]
    fn index(&self, index: Range<usize>) -> &[T]
    {
        assert!(index.start < index.end);
        assert!(index.end <= self.size);
        return unsafe
        {
            slice::from_raw_parts(self.ptr.offset(index.start as isize), index.end - index.start)
        };
    }
}

impl<T, A: Allocator> IndexMut<Range<usize>> for Array<T, A>
{
    #[inline]
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T]
    {
        assert!(index.start < index.end);
        assert!(index.end <= self.size);
        return unsafe
        {
            slice::from_raw_parts_mut(self.ptr.offset(index.start as isize), index.end - index.start)
        };
    }
}

impl<T, A: Allocator> Index<RangeInclusive<usize>> for Array<T, A>
{
    type Output = [T];

    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &[T]
    {
        assert!(*index.start() <= *index.end());
        return &self[*index.start()..(*index.end() + 1)];
    }
}

impl<T, A: Allocator> IndexMut<RangeInclusive<usize>> for Array<T, A>
{
    #[inline]
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut [T]
    {
        assert!(*index.start() <= *index.end());
        return &mut self[*index.start()..(*index.end() + 1)];
    }
}

impl<T, A: Allocator> Drop for Array<T, A>
{
    fn drop(&mut self)
    {
        if !self.ptr.is_null()
        {
            self.clear();

            unsafe
            {
                self.alloc.dealloc_aligned(self.ptr as *mut c_void);
            }
        }
    }
}

impl<T, A: Allocator> Deref for Array<T, A>
{
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        unsafe
        {
            slice::from_raw_parts(self.ptr, self.size)
        }
    }
}

impl<T, A: Allocator> DerefMut for Array<T, A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        unsafe
        {
            slice::from_raw_parts_mut(self.ptr, self.size)
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::Win32HeapAllocator;

    struct DropCheck<'a>
    {
        pub dropped: &'a mut bool,
    }

    impl<'a> DropCheck<'a>
    {
        fn new(b: &'a mut bool) -> Self
        {
            Self{ dropped: b}
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
        let alloc = Win32HeapAllocator::default();
        let mut a = Array::new(&alloc);

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
        let alloc = Win32HeapAllocator::default();
        let mut dropped = false;

        {
            let mut a = Array::new(&alloc);
            a.push(DropCheck::new(&mut dropped));
        }

        assert!(dropped);
    }

    fn sum_slice(slice: &[i32]) -> i32
    {
        slice.iter().sum()
    }

    fn double_slice(slice: &mut[i32])
    {
        slice.iter_mut()
            .for_each(|x| *x = *x * 2);
    }

    #[test]
    fn slice()
    {
        let alloc = Win32HeapAllocator::default();
        let mut a = Array::new(&alloc);

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
        let alloc = Win32HeapAllocator::default();
        let mut a = Array::new(&alloc);

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
        let alloc = Win32HeapAllocator::default();
        let mut a = Array::new(&alloc);

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
        let alloc = Win32HeapAllocator::default();
        let mut a = Array::new(&alloc);

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
}
