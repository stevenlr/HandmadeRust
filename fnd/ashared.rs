use crate::alloc::{alloc_one, Allocator, GlobalAllocator};
use core::{
    borrow::Borrow,
    convert::AsRef,
    marker::{PhantomData, Send, Sync, Unsize},
    ops::{CoerceUnsized, Deref},
    ptr::{drop_in_place, write, NonNull},
    sync::atomic::{AtomicUsize, Ordering::*},
};

struct ASharedInner<T: ?Sized>
{
    strong_count: AtomicUsize,
    weak_count:   AtomicUsize,
    value:        T,
}

impl<T: ?Sized> ASharedInner<T>
{
    #[inline]
    fn inc_strong(&self) -> usize
    {
        self.strong_count.fetch_add(1, SeqCst) + 1
    }

    #[inline]
    fn dec_strong(&self) -> usize
    {
        self.strong_count.fetch_sub(1, SeqCst) - 1
    }

    #[inline]
    fn strong_count(&self) -> usize
    {
        self.strong_count.load(SeqCst)
    }

    #[inline]
    fn inc_weak(&self) -> usize
    {
        self.weak_count.fetch_add(1, SeqCst) + 1
    }

    #[inline]
    fn dec_weak(&self) -> usize
    {
        self.weak_count.fetch_sub(1, SeqCst) - 1
    }
}

pub struct AShared<T: ?Sized, A: Allocator = GlobalAllocator>
{
    ptr:      NonNull<ASharedInner<T>>,
    alloc:    A,
    _phantom: PhantomData<T>,
}

pub struct AWeak<T: ?Sized, A: Allocator>
{
    ptr:   NonNull<ASharedInner<T>>,
    alloc: A,
}

unsafe impl<T, A> Send for AShared<T, A>
where
    T: ?Sized + Send + Sync + 'static,
    A: Allocator + Send + Sync + 'static,
{
}

unsafe impl<T, A> Sync for AShared<T, A>
where
    T: ?Sized + Send + Sync + 'static,
    A: Allocator + Send + Sync + 'static,
{
}

unsafe impl<T, A> Send for AWeak<T, A>
where
    T: ?Sized + Send + Sync + 'static,
    A: Allocator + Send + Sync + 'static,
{
}

unsafe impl<T, A> Sync for AWeak<T, A>
where
    T: ?Sized + Send + Sync + 'static,
    A: Allocator + Send + Sync + 'static,
{
}

impl<T, A> AShared<T, A>
where
    A: Allocator,
{
    pub fn new_with(value: T, mut alloc: A) -> Self
    {
        let mut ptr =
            unsafe { alloc_one::<ASharedInner<T>>(&mut alloc).expect("Allocation error") };

        unsafe {
            write(
                ptr.as_mut(),
                ASharedInner {
                    strong_count: AtomicUsize::new(1),
                    weak_count: AtomicUsize::new(1),
                    value,
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

impl<T, A> AShared<T, A>
where
    A: Allocator + Clone,
{
    pub fn downgrade(this: &Self) -> AWeak<T, A>
    {
        let inner = unsafe { this.ptr.as_ref() };
        inner.inc_weak();

        AWeak {
            ptr:   this.ptr,
            alloc: this.alloc.clone(),
        }
    }
}

impl<T, A> AWeak<T, A>
where
    A: Allocator + Clone,
{
    pub fn upgrade(&self) -> Option<AShared<T, A>>
    {
        let inner = unsafe { self.ptr.as_ref() };

        let mut strong = inner.strong_count();
        loop
        {
            if strong == 0
            {
                return None;
            }

            match inner
                .strong_count
                .compare_exchange(strong, strong + 1, SeqCst, SeqCst)
            {
                Ok(_) =>
                {
                    return Some(AShared {
                        ptr:      self.ptr,
                        alloc:    self.alloc.clone(),
                        _phantom: PhantomData,
                    })
                }
                Err(n) => strong = n,
            }
        }
    }
}

impl<T> AShared<T, GlobalAllocator>
{
    pub fn new(value: T) -> Self
    {
        Self::new_with(value, GlobalAllocator)
    }
}

impl<T, A> Clone for AShared<T, A>
where
    T: ?Sized,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self
    {
        let inner = unsafe { self.ptr.as_ref() };
        inner.inc_strong();

        Self {
            ptr:      self.ptr,
            alloc:    self.alloc.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T, A> Clone for AWeak<T, A>
where
    T: ?Sized,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self
    {
        let inner = unsafe { self.ptr.as_ref() };
        inner.inc_weak();

        Self {
            ptr:   self.ptr,
            alloc: self.alloc.clone(),
        }
    }
}

impl<T, A> Drop for AShared<T, A>
where
    T: ?Sized,
    A: Allocator,
{
    fn drop(&mut self)
    {
        let inner = unsafe { self.ptr.as_mut() };

        if inner.dec_strong() == 0
        {
            unsafe {
                drop_in_place(&mut inner.value);
            }

            if inner.dec_weak() == 0
            {
                unsafe {
                    self.alloc.dealloc_aligned(self.ptr.cast().as_ptr());
                }
            }
        }
    }
}

impl<T, A> Drop for AWeak<T, A>
where
    T: ?Sized,
    A: Allocator,
{
    fn drop(&mut self)
    {
        let inner = unsafe { self.ptr.as_mut() };

        if inner.dec_weak() == 0
        {
            unsafe {
                self.alloc.dealloc_aligned(self.ptr.cast().as_ptr());
            }
        }
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<AShared<U, A>> for AShared<T, A> {}

impl<T: ?Sized, A: Allocator> Unpin for AShared<T, A> {}

impl<T: ?Sized, A: Allocator> Deref for AShared<T, A>
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target
    {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T: ?Sized, A: Allocator> AsRef<T> for AShared<T, A>
{
    #[inline]
    fn as_ref(&self) -> &T
    {
        self
    }
}

impl<T: ?Sized, A: Allocator> Borrow<T> for AShared<T, A>
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
    use core::{
        cell::{Cell, RefCell},
        mem::drop,
    };

    struct MyObject<'a>
    {
        x:       i32,
        y:       i32,
        s:       &'static str,
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
        let dropped = Cell::new(false);

        {
            let p = AShared::new(RefCell::new(MyObject {
                x:       1,
                y:       2,
                s:       "hello",
                dropped: &dropped,
            }));

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
        let dropped = Cell::new(false);

        let p = AShared::new(RefCell::new(MyObject {
            x:       1,
            y:       2,
            s:       "hello",
            dropped: &dropped,
        }));

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
        let dropped = Cell::new(false);

        let p = AShared::new(RefCell::new(MyObject {
            x:       1,
            y:       2,
            s:       "hello",
            dropped: &dropped,
        }));

        assert!(!dropped.get());

        let p2 = p.clone();
        assert!(!dropped.get());
        assert!(p.as_ref().as_ptr() == p2.as_ref().as_ptr());

        let w = AShared::downgrade(&p2);
        assert!(w.upgrade().is_some());
        assert!(w.upgrade().unwrap().as_ref().borrow().s == "hello");

        drop(p);
        drop(w);
        assert!(!dropped.get());

        let w = AShared::downgrade(&p2);
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

    fn create_dst(x: i32) -> AShared<dyn MyTrait>
    {
        AShared::new(MyObject2 { x })
    }

    #[test]
    fn dst()
    {
        let my_dst = create_dst(42);
        assert!(my_dst.do_something() == 42);
    }

    fn create_closure(y: i32) -> AShared<dyn Fn(i32) -> i32>
    {
        AShared::new(move |x| x + y)
    }

    #[test]
    fn closure()
    {
        let closure = create_closure(5);

        assert!(closure(5) == 10);
        assert!(closure(6) == 11);
    }
}
