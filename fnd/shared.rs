use crate::alloc::{alloc_one, Allocator, GlobalAllocator};
use core::{
    borrow::Borrow,
    cell::Cell,
    convert::AsRef,
    marker::{PhantomData, Send, Sync, Unsize},
    ops::{CoerceUnsized, Deref},
    ptr::{drop_in_place, write, NonNull},
};

struct SharedInner<T: ?Sized>
{
    strong_count: Cell<usize>,
    weak_count: Cell<usize>,
    value: T,
}

impl<T: ?Sized> SharedInner<T>
{
    #[inline]
    fn inc_strong(&self)
    {
        self.strong_count.set(self.strong_count.get() + 1);
    }

    #[inline]
    fn dec_strong(&self)
    {
        self.strong_count.set(self.strong_count.get() - 1);
    }

    #[inline]
    fn strong_count(&self) -> usize
    {
        self.strong_count.get()
    }

    #[inline]
    fn inc_weak(&self)
    {
        self.weak_count.set(self.weak_count.get() + 1);
    }

    #[inline]
    fn dec_weak(&self)
    {
        self.weak_count.set(self.weak_count.get() - 1);
    }

    #[inline]
    fn weak_count(&self) -> usize
    {
        self.weak_count.get()
    }
}

pub struct Shared<T: ?Sized, A: Allocator = GlobalAllocator>
{
    ptr: NonNull<SharedInner<T>>,
    alloc: A,
    _phantom: PhantomData<T>,
}

pub struct Weak<T: ?Sized, A: Allocator>
{
    ptr: NonNull<SharedInner<T>>,
    alloc: A,
}

impl<T, A> !Send for Shared<T, A>
where
    T: ?Sized,
    A: Allocator,
{
}

impl<T, A> !Sync for Shared<T, A>
where
    T: ?Sized,
    A: Allocator,
{
}

impl<T, A> !Send for Weak<T, A>
where
    T: ?Sized,
    A: Allocator,
{
}

impl<T, A> !Sync for Weak<T, A>
where
    T: ?Sized,
    A: Allocator,
{
}

impl<T, A> Shared<T, A>
where
    A: Allocator,
{
    pub fn new_with(value: T, mut alloc: A) -> Self
    {
        let mut ptr = unsafe { alloc_one::<SharedInner<T>>(&mut alloc).expect("Allocation error") };

        unsafe {
            write(
                ptr.as_mut(),
                SharedInner {
                    strong_count: Cell::new(1),
                    weak_count: Cell::new(1),
                    value: value,
                },
            );
        }

        return Self {
            ptr,
            alloc,
            _phantom: PhantomData,
        };
    }
}

impl<T, A> Shared<T, A>
where
    A: Allocator + Clone,
{
    pub fn downgrade(this: &Self) -> Weak<T, A>
    {
        let inner = unsafe { this.ptr.as_ref() };
        inner.inc_weak();

        Weak {
            ptr: this.ptr,
            alloc: this.alloc.clone(),
        }
    }
}

impl<T, A> Weak<T, A>
where
    A: Allocator + Clone,
{
    pub fn upgrade(&self) -> Option<Shared<T, A>>
    {
        let inner = unsafe { self.ptr.as_ref() };

        if inner.strong_count() == 0
        {
            return None;
        }
        else
        {
            inner.inc_strong();
            return Some(Shared {
                ptr: self.ptr,
                alloc: self.alloc.clone(),
                _phantom: PhantomData,
            });
        }
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

        Self {
            ptr: self.ptr,
            alloc: self.alloc.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, A> Clone for Weak<T, A>
where
    T: ?Sized,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self
    {
        let inner = unsafe { self.ptr.as_ref() };
        inner.inc_weak();

        Self {
            ptr: self.ptr,
            alloc: self.alloc.clone(),
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
        let inner = unsafe { self.ptr.as_mut() };

        inner.dec_strong();
        if inner.strong_count() == 0
        {
            unsafe {
                drop_in_place(&mut inner.value);
            }

            inner.dec_weak();
            if inner.weak_count() == 0
            {
                unsafe {
                    self.alloc.dealloc_aligned(self.ptr.cast().as_ptr());
                }
            }
        }
    }
}

impl<T, A> Drop for Weak<T, A>
where
    T: ?Sized,
    A: Allocator,
{
    fn drop(&mut self)
    {
        let inner = unsafe { self.ptr.as_mut() };

        inner.dec_weak();
        if inner.weak_count() == 0
        {
            unsafe {
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
        unsafe { &self.ptr.as_ref().value }
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

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::SystemAllocator;
    use core::{cell::RefCell, mem::drop};

    struct MyObject<'a>
    {
        x: i32,
        y: i32,
        s: &'static str,
        dropped: &'a Cell<bool>,
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
            self.dropped.set(true);
        }
    }

    #[test]
    fn simple()
    {
        let alloc = SystemAllocator::default();
        let dropped = Cell::new(false);

        {
            let p = Shared::new_with(
                RefCell::new(MyObject {
                    x: 1,
                    y: 2,
                    s: "hello",
                    dropped: &dropped,
                }),
                &alloc,
            );

            assert!(p.as_ref().borrow().x == 1);
            assert!(p.as_ref().borrow().y == 2);
            assert!(p.as_ref().borrow().s == "hello");

            p.as_ref().borrow_mut().x = 45;
            assert!(p.as_ref().borrow().x == 45);
            assert!(p.as_ref().borrow().y == 2);
            assert!(p.as_ref().borrow().s == "hello");

            assert!(p.as_ref().borrow().add() == 47);

            p.as_ref().borrow_mut().set(8, 9);
            assert!(p.as_ref().borrow().x == 8);
            assert!(p.as_ref().borrow().y == 9);
            assert!(p.as_ref().borrow().s == "hello");

            assert!(!dropped.get());
        }

        assert!(dropped.get());
    }

    #[test]
    fn multiple_references()
    {
        let alloc = SystemAllocator::default();
        let dropped = Cell::new(false);

        let p = Shared::new_with(
            RefCell::new(MyObject {
                x: 1,
                y: 2,
                s: "hello",
                dropped: &dropped,
            }),
            &alloc,
        );

        assert!(!dropped.get());

        let p2 = p.clone();
        assert!(!dropped.get());
        assert!(p.as_ref().as_ptr() == p2.as_ref().as_ptr());

        drop(p);
        assert!(!dropped.get());

        drop(p2);
        assert!(dropped.get());
    }

    #[test]
    fn multiple_references_weak()
    {
        let alloc = SystemAllocator::default();
        let dropped = Cell::new(false);

        let p = Shared::new_with(
            RefCell::new(MyObject {
                x: 1,
                y: 2,
                s: "hello",
                dropped: &dropped,
            }),
            &alloc,
        );

        assert!(!dropped.get());

        let p2 = p.clone();
        assert!(!dropped.get());
        assert!(p.as_ref().as_ptr() == p2.as_ref().as_ptr());

        let w = Shared::downgrade(&p2);
        assert!(w.upgrade().is_some());
        assert!(w.upgrade().unwrap().as_ref().borrow().s == "hello");

        drop(p);
        drop(w);
        assert!(!dropped.get());

        let w = Shared::downgrade(&p2);
        let w2 = w.clone();

        assert!(w.upgrade().is_some());
        assert!(w.upgrade().unwrap().as_ref().borrow().s == "hello");

        drop(w);

        assert!(w2.upgrade().is_some());
        assert!(w2.upgrade().unwrap().as_ref().borrow().s == "hello");

        drop(p2);
        assert!(dropped.get());

        assert!(w2.upgrade().is_none());
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

    fn create_dst<A: Allocator>(x: i32, alloc: A) -> Shared<dyn MyTrait, A>
    {
        Shared::new_with(MyObject2 { x }, alloc)
    }

    #[test]
    fn dst()
    {
        let alloc = SystemAllocator::default();
        let my_dst = create_dst(42, &alloc);
        assert!(my_dst.do_something() == 42);
    }

    fn create_closure<A: Allocator>(y: i32, alloc: A) -> Shared<Fn(i32) -> i32, A>
    {
        Shared::new_with(move |x| x + y, alloc)
    }

    #[test]
    fn closure()
    {
        let alloc = SystemAllocator::default();
        let closure = create_closure(5, &alloc);

        assert!(closure(5) == 10);
        assert!(closure(6) == 11);
    }
}
