use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::Array,
};

use core::{
    borrow::Borrow,
    mem::{needs_drop, ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr, slice,
};

pub trait StackArray
{
    type Element;

    fn len(&self) -> usize;
    fn as_ptr(&self) -> *const Self::Element;
    fn as_mut_ptr(&mut self) -> *mut Self::Element;
}

enum SmallArrayData<S, A = GlobalAllocator>
where
    S: StackArray,
    A: Allocator,
{
    Stack(usize, ManuallyDrop<S>),
    Heap(Array<S::Element, A>),
}

pub struct SmallArray<S, A = GlobalAllocator>
where
    S: StackArray,
    A: Allocator,
{
    alloc: Option<A>,
    data:  SmallArrayData<S, A>,
}

impl<S, A> SmallArray<S, A>
where
    S: StackArray,
    A: Allocator,
{
    #[inline]
    pub fn len(&self) -> usize
    {
        self.get_infos().1
    }

    #[inline]
    pub fn capacity(&self) -> usize
    {
        self.get_infos().2
    }

    pub fn reserve(&mut self, new_cap: usize)
    {
        if new_cap <= self.capacity()
        {
            return;
        }

        if let SmallArrayData::Stack(used, array) = &mut self.data
        {
            if new_cap > array.len()
            {
                let alloc = self.alloc.take().unwrap();
                let mut new_array = Array::new_with(alloc);
                new_array.reserve(new_cap);

                let ptr = array.as_mut_ptr();
                for i in 0..*used
                {
                    new_array.push(unsafe { ptr::read(ptr.offset(i as isize)) });
                }

                self.data = SmallArrayData::Heap(new_array);
            }
        }

        if let SmallArrayData::Heap(array) = &mut self.data
        {
            array.reserve(new_cap);
        }
    }

    pub fn new_with(alloc: A) -> Self
    {
        Self {
            alloc: Some(alloc),
            data:  SmallArrayData::Stack(0, unsafe { MaybeUninit::uninit().assume_init() }),
        }
    }

    #[inline]
    fn get_infos(&self) -> (*const S::Element, usize, usize)
    {
        match &self.data
        {
            SmallArrayData::Stack(used, array) => (array.as_ptr(), *used, array.len()),
            SmallArrayData::Heap(array) => (array.as_ptr(), array.len(), array.capacity()),
        }
    }

    #[inline]
    fn get_infos_mut(&mut self) -> (*mut S::Element, usize, usize)
    {
        match &mut self.data
        {
            SmallArrayData::Stack(used, array) => (array.as_mut_ptr(), *used, array.len()),
            SmallArrayData::Heap(array) => (array.as_mut_ptr(), array.len(), array.capacity()),
        }
    }

    pub fn push(&mut self, element: S::Element)
    {
        let (_, len, cap) = self.get_infos_mut();
        if len == cap
        {
            self.reserve(len * 2);
        }

        match &mut self.data
        {
            SmallArrayData::Stack(used, array) =>
            {
                unsafe {
                    ptr::write(array.as_mut_ptr().offset(*used as isize), element);
                }
                *used += 1;
            }
            SmallArrayData::Heap(array) =>
            {
                array.push(element);
            }
        }
    }

    pub fn pop(&mut self) -> Option<S::Element>
    {
        if self.len() == 0
        {
            return None;
        }

        match &mut self.data
        {
            SmallArrayData::Stack(used, array) =>
            {
                *used -= 1;
                unsafe { Some(ptr::read(array.as_mut_ptr().offset(*used as isize))) }
            }
            SmallArrayData::Heap(array) => array.pop(),
        }
    }

    pub fn clear(&mut self)
    {
        match &mut self.data
        {
            SmallArrayData::Stack(used, array) =>
            {
                if needs_drop::<S::Element>()
                {
                    for i in 0..*used
                    {
                        unsafe {
                            ptr::drop_in_place(array.as_mut_ptr().offset(i as isize));
                        }
                    }
                }
                *used = 0;
            }
            SmallArrayData::Heap(array) => array.clear(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
        self.len() == 0
    }

    pub fn resize_with<F>(&mut self, new_size: usize, f: F)
    where
        F: Fn() -> S::Element,
    {
        self.reserve(new_size);

        let (ptr, len, _) = self.get_infos_mut();

        match &mut self.data
        {
            SmallArrayData::Stack(used, _) =>
            {
                if new_size < len && needs_drop::<S::Element>()
                {
                    for i in new_size..len
                    {
                        unsafe {
                            ptr::drop_in_place(ptr.offset(i as isize));
                        }
                    }
                }
                else if new_size > len
                {
                    for i in len..new_size
                    {
                        unsafe {
                            ptr::write(ptr.offset(i as isize), f());
                        }
                    }
                }

                *used = new_size;
            }
            SmallArrayData::Heap(array) => array.resize_with(new_size, f),
        }
    }

    pub fn resize(&mut self, new_size: usize, value: S::Element)
    where
        S::Element: Clone,
    {
        self.resize_with(new_size, || value.clone());
    }

    pub fn resize_default(&mut self, new_size: usize)
    where
        S::Element: Default,
    {
        self.resize_with(new_size, || S::Element::default());
    }
}

impl<S, A> Drop for SmallArray<S, A>
where
    S: StackArray,
    A: Allocator,
{
    fn drop(&mut self)
    {
        self.clear();
    }
}

impl<S, A> Deref for SmallArray<S, A>
where
    S: StackArray,
    A: Allocator,
{
    type Target = [S::Element];

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        let (ptr, len, _) = self.get_infos();
        unsafe { slice::from_raw_parts(ptr, len) }
    }
}

impl<S, A> DerefMut for SmallArray<S, A>
where
    S: StackArray,
    A: Allocator,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        let (ptr, len, _) = self.get_infos_mut();
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
}

impl<S> SmallArray<S, GlobalAllocator>
where
    S: StackArray,
{
    pub fn new() -> Self
    {
        Self::new_with(GlobalAllocator)
    }
}

impl<T, S, A: Allocator> Extend<T> for SmallArray<S, A>
where
    T: Borrow<S::Element>,
    S: StackArray,
    S::Element: Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for e in iter
        {
            self.push(e.borrow().clone());
        }
    }
}

macro_rules! impl_stack_array {
    ($len:expr, $name:ident) => {
        impl<T> StackArray for [T; $len]
        {
            type Element = T;

            #[inline]
            fn len(&self) -> usize
            {
                $len
            }

            #[inline]
            fn as_ptr(&self) -> *const Self::Element
            {
                (self as &[Self::Element]).as_ptr()
            }

            #[inline]
            fn as_mut_ptr(&mut self) -> *mut Self::Element
            {
                (self as &mut [Self::Element]).as_mut_ptr()
            }
        }

        pub type $name<T, A = GlobalAllocator> = SmallArray<[T; $len], A>;
    };
}

// @Todo Re-do this when const generics
impl_stack_array!(1, SmallArray1);
impl_stack_array!(2, SmallArray2);
impl_stack_array!(4, SmallArray4);
impl_stack_array!(8, SmallArray8);
impl_stack_array!(16, SmallArray16);
impl_stack_array!(24, SmallArray24);
impl_stack_array!(32, SmallArray32);
impl_stack_array!(64, SmallArray64);
impl_stack_array!(128, SmallArray128);

#[cfg(test)]
mod tests
{
    use super::*;
    use core::cell::Cell;

    struct DropCheck<'a>
    {
        pub dropped: &'a Cell<i32>,
    }

    impl<'a> DropCheck<'a>
    {
        fn new(b: &'a Cell<i32>) -> Self
        {
            Self { dropped: b }
        }
    }

    impl<'a> Drop for DropCheck<'a>
    {
        fn drop(&mut self)
        {
            self.dropped.set(self.dropped.get() + 1);
        }
    }

    macro_rules! test_size {
        ($name:ident, $type:ty) => {
            mod $name
            {
                use super::*;

                #[test]
                fn push_pop()
                {
                    let mut a = <$type>::new();

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
                    let dropped = Cell::new(0);

                    {
                        let mut a = <$type>::new();
                        a.push(DropCheck::new(&dropped));
                    }

                    assert!(dropped.get() == 1);
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
                    let mut a = <$type>::new();

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
                    let mut a = <$type>::new();

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
                    let mut a = <$type>::new();

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
                    let mut a = <$type>::new();

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
                    let mut a = <$type>::new();

                    a.push(1);
                    a.extend([7, 8].into_iter());
                    a.extend(&[2, 3]);
                    a.extend(Some(10));

                    assert!(a[0] == 1);
                    assert!(a[1] == 7);
                    assert!(a[2] == 8);
                    assert!(a[3] == 2);
                    assert!(a[4] == 3);
                    assert!(a[5] == 10);
                }

                #[test]
                fn zst()
                {
                    let mut a = <$type>::new();

                    a.push(());
                    a.push(());
                    a.push(());
                    assert!(a.len() == 3);

                    assert!(a[1] == ());

                    a.clear();
                    assert!(a.len() == 0);
                }
            }
        };
    }

    test_size!(size_1, SmallArray1<_, _>);
    test_size!(size_2, SmallArray2<_, _>);
    test_size!(size_4, SmallArray4<_, _>);
    test_size!(size_8, SmallArray8<_, _>);
    test_size!(size_16, SmallArray16<_, _>);
}
