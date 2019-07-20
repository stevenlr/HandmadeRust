use core::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

use win32::{
    kernel32::{AcquireSRWLockExclusive, InitializeSRWLock, ReleaseSRWLockExclusive},
    SRWLOCK,
};

pub struct LockGuard<'a, T>
{
    mutex: &'a Mutex<T>,
}

pub struct Mutex<T>
{
    handle: UnsafeCell<SRWLOCK>,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T>
{
    pub fn new(value: T) -> Self
    {
        let mut handle = MaybeUninit::uninit();

        unsafe {
            InitializeSRWLock(handle.as_mut_ptr());
        }

        Self {
            handle: UnsafeCell::new(unsafe { handle.assume_init() }),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> LockGuard<'a, T>
    {
        unsafe {
            AcquireSRWLockExclusive(self.handle.get());
        }

        LockGuard { mutex: &self }
    }
}

unsafe impl<T> Send for Mutex<T> {}
unsafe impl<T> Sync for Mutex<T> {}

impl<'a, T> Drop for LockGuard<'a, T>
{
    fn drop(&mut self)
    {
        unsafe {
            ReleaseSRWLockExclusive(self.mutex.handle.get());
        }
    }
}

impl<'a, T> Deref for LockGuard<'a, T>
{
    type Target = T;

    fn deref(&self) -> &T
    {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'a, T> DerefMut for LockGuard<'a, T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        unsafe { &mut *self.mutex.value.get() }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::{thread::Thread, AShared};

    #[test]
    fn mutex()
    {
        let shared = AShared::new(Mutex::new(0i32));

        let sa = shared.clone();
        let a = Thread::spawn(move || {
            for _ in 0..10000
            {
                let mut lock = sa.lock();
                *lock += 1;
            }
        });

        let sb = shared.clone();
        let b = Thread::spawn(move || {
            for _ in 0..10000
            {
                let mut lock = sb.lock();
                *lock += 1;
            }
        });

        a.join();
        b.join();

        assert_eq!(*shared.lock(), 20000);
    }
}
