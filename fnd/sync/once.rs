use core::{
    mem::MaybeUninit,
    sync::atomic::{spin_loop_hint, AtomicUsize, Ordering::*},
};

const UNINIT: usize = 0;
const RUNNING: usize = 1;
const FINISHED: usize = 2;

pub struct Once<T>
{
    state: AtomicUsize,
    value: MaybeUninit<T>,
}

unsafe impl<T: Send> Send for Once<T> {}
unsafe impl<T: Send + Sync> Sync for Once<T> {}

impl<T> Once<T>
{
    pub const fn new() -> Self
    {
        Self {
            state: AtomicUsize::new(0),
            value: MaybeUninit::uninit(),
        }
    }

    pub fn get<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        if self.state.compare_and_swap(UNINIT, RUNNING, Relaxed) == UNINIT
        {
            unsafe {
                (self.value.as_ptr() as *mut T).write(f());
            }

            self.state.store(FINISHED, Release);
        }

        loop
        {
            if self.state.load(Acquire) == FINISHED
            {
                return unsafe { &*self.value.as_ptr() };
            }
            spin_loop_hint();
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use core::cell::Cell;

    #[test]
    fn once()
    {
        let i = Cell::new(0);
        let once = Once::new();

        let a = once.get(|| {
            i.set(i.get() + 1);
            return i.get();
        });

        let b = once.get(|| {
            i.set(i.get() + 1);
            return i.get();
        });

        assert_eq!(a, b);
        assert_eq!(*a, 1);
    }

    #[test]
    fn once_static()
    {
        let i = Cell::new(0);
        static ONCE: Once<i32> = Once::new();

        let a = ONCE.get(|| {
            i.set(i.get() + 1);
            return i.get();
        });

        let b = ONCE.get(|| {
            i.set(i.get() + 1);
            return i.get();
        });

        assert_eq!(a, b);
        assert_eq!(*a, 1);
    }
}
