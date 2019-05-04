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
        if self.full || self.buffer.capacity == 0
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
            self.full = false;

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

        self.full = false;
        self.head = 0;
        self.tail = 0;
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

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::Win32HeapAllocator;
    use core::cell::Cell;

    struct Droppable<'a>
    {
        pub dropped: &'a Cell<bool>,
    }

    impl<'a> Drop for Droppable<'a>
    {
        fn drop(&mut self)
        {
            self.dropped.set(true);
        }
    }

    #[test]
    fn simple()
    {
        let alloc = Win32HeapAllocator::default();
        let mut q = Queue::new_with(&alloc);

        assert!(q.is_empty());
        assert!(q.len() == 0);
        assert!(q.peek().is_none());
        assert!(q.peek_mut().is_none());
        assert!(q.pop().is_none());

        q.push(1);
        assert!(!q.is_empty());
        assert!(q.len() == 1);
        assert!(q.peek() == Some(&1));
        assert!(q.peek_mut() == Some(&mut 1));

        q.push(2);
        assert!(!q.is_empty());
        assert!(q.len() == 2);
        assert!(q.peek() == Some(&1));
        assert!(q.peek_mut() == Some(&mut 1));

        *q.peek_mut().unwrap() = 3;
        assert!(!q.is_empty());
        assert!(q.len() == 2);
        assert!(q.peek() == Some(&3));
        assert!(q.peek_mut() == Some(&mut 3));

        assert!(q.pop() == Some(3));
        assert!(!q.is_empty());
        assert_eq!(q.len(), 1);
        assert!(q.peek() == Some(&2));
        assert!(q.peek_mut() == Some(&mut 2));

        assert!(q.pop() == Some(2));
        assert!(q.is_empty());
        assert!(q.len() == 0);
        assert!(q.peek().is_none());
        assert!(q.peek_mut().is_none());
        assert!(q.pop().is_none());
    }

    #[test]
    fn wrap_around()
    {
        let alloc = Win32HeapAllocator::default();
        let mut q = Queue::new_with(&alloc);

        q.reserve(3);
        q.push(1);
        q.push(2);
        q.push(3);

        // [1 2 3]
        assert_eq!(q.pop(), Some(1));

        // x [2 3]
        q.push(4);

        // 4] [2 3
        assert_eq!(q.pop(), Some(2));

        // 4] x [3
        q.push(5);

        // [4 5] x
        assert_eq!(q.pop(), Some(3));

        // x [5] x
        assert_eq!(q.pop(), Some(4));

        // x x x
        assert_eq!(q.pop(), Some(5));
    }

    #[test]
    fn clear()
    {
        let alloc = Win32HeapAllocator::default();
        let mut q = Queue::new_with(&alloc);

        q.reserve(3);
        q.push(1);
        q.push(2);
        q.push(3);

        q.clear();
        assert!(q.is_empty());
        assert!(q.len() == 0);
        assert!(q.peek().is_none());
        assert!(q.peek_mut().is_none());
        assert!(q.pop().is_none());
    }

    #[test]
    fn drop()
    {
        let alloc = Win32HeapAllocator::default();

        let d1 = Cell::new(false);
        let d2 = Cell::new(false);
        let d3 = Cell::new(false);

        let mut q = Queue::new_with(&alloc);

        q.push(Droppable { dropped: &d1 });
        q.push(Droppable { dropped: &d2 });
        q.push(Droppable { dropped: &d3 });

        assert!(!d1.get());
        assert!(!d2.get());
        assert!(!d3.get());

        q.pop();

        assert!(d1.get());
        assert!(!d2.get());
        assert!(!d3.get());

        q.clear();

        assert!(d1.get());
        assert!(d2.get());
        assert!(d3.get());
    }

    #[test]
    fn reserve_normal()
    {
        let alloc = Win32HeapAllocator::default();
        let mut q = Queue::new_with(&alloc);

        q.reserve(3);
        q.push(1);
        q.push(2);
        q.push(3);
        q.pop();

        // x [2 3]
        q.reserve(12);

        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), Some(3));
        assert!(q.is_empty());
    }

    #[test]
    fn reserve_wrap()
    {
        let alloc = Win32HeapAllocator::default();
        let mut q = Queue::new_with(&alloc);

        q.reserve(3);
        q.push(1);
        q.push(2);
        q.push(3);
        q.pop();
        q.push(4);
        q.pop();

        // 4] x [3
        q.reserve(12);

        assert_eq!(q.pop(), Some(3));
        assert_eq!(q.pop(), Some(4));
        assert!(q.is_empty());
    }

    #[test]
    fn zst()
    {
        let alloc = Win32HeapAllocator::default();
        let mut q = Queue::new_with(&alloc);

        q.push(());
        q.push(());
        q.push(());
        assert!(q.len() == 3);

        q.pop();
        q.pop();
        assert!(q.len() == 1);

        q.push(());
        q.push(());
        q.push(());
        assert!(q.len() == 4);

        q.clear();
        assert!(q.is_empty());
    }
}
