use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::Array,
};

use core::{
    mem::{needs_drop, replace, ManuallyDrop, MaybeUninit},
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
    data: SmallArrayData<S, A>,
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
                let alloc = replace(&mut self.alloc, None).unwrap();
                let mut new_array = Array::new_with(alloc);
                new_array.reserve(new_cap);

                let ptr = array.as_mut_ptr();
                for i in 0..*used
                {
                    new_array.push(unsafe { ptr::read(ptr.offset(i as isize)) });
                }

                replace(&mut self.data, SmallArrayData::Heap(new_array));
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
            data: SmallArrayData::Stack(0, unsafe { MaybeUninit::uninit().assume_init() }),
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

impl<T> StackArray for [T; 1]
{
    type Element = T;

    #[inline]
    fn len(&self) -> usize
    {
        1
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

// @Todo Implement StackArray for a bunch of lengths (until const generics)
// @Todo Test SmallArray
