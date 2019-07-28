use core::{
    cell::UnsafeCell,
    marker::Unpin,
    ops::{Deref, DerefMut},
    pin::Pin,
};

use crate::Unq;

use win32::{
    kernel32::{AcquireSRWLockExclusive, ReleaseSRWLockExclusive},
    SRWLOCK, SRWLOCK_INIT,
};

pub struct LockGuard<'a, T>
{
    mutex: &'a Mutex<T>,
}

struct NativeMutex
{
    handle: UnsafeCell<SRWLOCK>,
}

impl Unpin for NativeMutex {}

impl NativeMutex
{
    fn new() -> Self
    {
        Self {
            handle: UnsafeCell::new(SRWLOCK_INIT),
        }
    }

    fn lock(&self)
    {
        unsafe {
            AcquireSRWLockExclusive(self.handle.get());
        }
    }

    fn unlock(&self)
    {
        unsafe {
            ReleaseSRWLockExclusive(self.handle.get());
        }
    }
}

pub struct Mutex<T>
{
    native: Pin<Unq<NativeMutex>>,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T>
{
    pub fn new(value: T) -> Self
    {
        Self {
            native: Unq::pin(NativeMutex::new()),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> LockGuard<'a, T>
    {
        self.native.lock();
        LockGuard { mutex: &self }
    }
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<'a, T> Drop for LockGuard<'a, T>
{
    fn drop(&mut self)
    {
        self.mutex.native.unlock();
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
