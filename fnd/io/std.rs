use super::{Error, Result, Write};

use core::ptr::null_mut;

use crate::sync::{Mutex, Once};

use win32::{
    kernel32::{FlushFileBuffers, GetStdHandle, WriteFile},
    HANDLE, STD_OUTPUT_HANDLE,
};

pub struct Stdout
{
    handle: Mutex<HANDLE>,
}

unsafe impl Send for Stdout {}
unsafe impl Sync for Stdout {}

impl Stdout
{
    fn new() -> Self
    {
        Self {
            handle: Mutex::new(unsafe { GetStdHandle(STD_OUTPUT_HANDLE) }),
        }
    }
}

impl Write for &Stdout
{
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        if buf.len() > core::u32::MAX as usize
        {
            return Err(Error::BufferTooLarge);
        }

        let handle = self.handle.lock();

        let mut bytes_written = 0;

        let success = unsafe {
            WriteFile(
                *handle,
                buf.as_ptr() as _,
                buf.len() as _,
                &mut bytes_written,
                null_mut(),
            ) != 0
        };

        match success
        {
            true => Ok(bytes_written as _),
            false => Err(Error::CannotWrite),
        }
    }

    fn flush(&mut self) -> Result<()>
    {
        let handle = self.handle.lock();

        let success = unsafe { FlushFileBuffers(*handle) != 0 };

        match success
        {
            true => Ok(()),
            false => Err(Error::CannotFlush),
        }
    }
}

static STDOUT_ONCE: Once<Stdout> = Once::new();

pub fn stdout() -> &'static Stdout
{
    STDOUT_ONCE.get(|| Stdout::new())
}
