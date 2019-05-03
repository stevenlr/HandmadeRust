use core::mem::needs_drop;
use core::ptr::drop_in_place;

use crate::alloc::{Allocator, GlobalAllocator, Layout};
use crate::containers::RawArray;

pub struct Queue<T, A: Allocator + Clone = GlobalAllocator>
{
    buffer: RawArray<T, A>,
    head: usize,
    tail: usize,
    full: bool,
}

impl<T, A> Queue<T, A>
where
    A: Allocator + Clone,
{
    pub fn new_with(alloc: A) -> Self
    {
        Self {
            buffer: RawArray::new(alloc),
            head: 0,
            tail: 0,
            full: false,
        }
    }

    #[inline]
    fn increment_wrap(&self, i: usize) -> usize
    {
        if i + 1 == self.buffer.capacity
        {
            0
        }
        else
        {
            i + 1
        }
    }

    pub fn len(&self) -> usize
    {
        if self.full
        {
            self.buffer.capacity
        }
        else if self.head < self.tail
        {
            self.buffer.capacity - self.tail + self.head
        }
        else
        {
            self.head - self.tail
        }
    }

    pub fn reserve(&mut self, new_capacity: usize)
    {
        if new_capacity > self.buffer.capacity
        {
            let mut new_buffer = RawArray::new(self.buffer.alloc.clone());
            new_buffer.reserve(new_capacity);

            let len = self.len();

            //  |---T###H---|
            //  |T###H-----------------|
            if self.head > self.tail
            {
                unsafe {
                    self.buffer
                        .ptr
                        .offset(self.tail as isize)
                        .copy_to(new_buffer.ptr, len);
                }
            }
            //  |##H------T#|
            //  |T###H-----------------|
            else if len > 0
            {
                unsafe {
                    self.buffer
                        .ptr
                        .offset(self.tail as isize)
                        .copy_to(new_buffer.ptr, self.buffer.capacity - self.tail);

                    self.buffer.ptr.copy_to(
                        new_buffer
                            .ptr
                            .offset((self.buffer.capacity - self.tail) as isize),
                        self.head,
                    );
                }
            }

            self.head = len;
            self.tail = 0;
            self.buffer = new_buffer;
        }
    }

    fn grow(&mut self)
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

    pub fn push(&mut self, value: T)
    {
        if self.full
        {
            self.grow();
        }

        unsafe {
            self.buffer.ptr.offset(self.head as isize).write(value);
        }

        self.head = self.increment_wrap(self.head);
        self.full = self.head == self.tail;
    }

    pub fn pop(&mut self) -> Option<T>
    {
        if self.is_empty()
        {
            None
        }
        else
        {
            let index = self.tail;

            self.tail = self.increment_wrap(self.tail);

            Some(unsafe { self.buffer.ptr.offset(index as isize).read() })
        }
    }

    pub fn peek(&self) -> Option<&T>
    {
        if self.is_empty()
        {
            None
        }
        else
        {
            unsafe { self.buffer.ptr.offset(self.tail as isize).as_ref() }
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T>
    {
        if self.is_empty()
        {
            None
        }
        else
        {
            unsafe { self.buffer.ptr.offset(self.tail as isize).as_mut() }
        }
    }

    pub fn clear(&mut self)
    {
        if !self.is_empty() && needs_drop::<T>()
        {
            let mut index = self.tail;

            loop
            {
                unsafe {
                    drop_in_place(self.buffer.ptr.offset(index as isize));
                }

                index = self.increment_wrap(index);

                if index == self.head
                {
                    break;
                }
            }
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
        !self.full && self.head == self.tail
    }
}

impl<T> Queue<T, GlobalAllocator>
{
    pub fn new() -> Self
    {
        Self::new_with(GlobalAllocator)
    }
}

impl<T, A> Drop for Queue<T, A>
where
    A: Allocator + Clone,
{
    fn drop(&mut self)
    {
        if !self.buffer.ptr.is_null()
        {
            self.clear();
        }
    }
}
