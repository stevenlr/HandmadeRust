use super::{containers::WString, io};

use win32::{
    kernel32::{CloseHandle, CreateFileW, FlushFileBuffers, ReadFile, SetFilePointerEx, WriteFile},
    *,
};

use core::ptr::null_mut;

mod private
{
    pub trait Sealed
    {
    }
}

#[derive(Default, Clone)]
pub struct OpenOptions
{
    read: bool,
    write: bool,
    truncate: bool,
    create: bool,
}

pub trait ToOpenOptions: private::Sealed
{
    fn to_open_options(&self) -> OpenOptions;
}

pub struct ReadOptions(OpenOptions);
pub struct WriteOptions(OpenOptions);

impl OpenOptions
{
    pub fn read() -> ReadOptions
    {
        ReadOptions(OpenOptions {
            read: true,
            ..Default::default()
        })
    }

    pub fn write() -> WriteOptions
    {
        WriteOptions(OpenOptions {
            write: true,
            ..Default::default()
        })
    }
}

impl ReadOptions
{
    #[inline]
    pub fn write(mut self) -> WriteOptions
    {
        self.0.write = true;
        WriteOptions(self.0)
    }
}

impl private::Sealed for ReadOptions {}
impl ToOpenOptions for ReadOptions
{
    #[inline]
    fn to_open_options(&self) -> OpenOptions
    {
        self.0.clone()
    }
}

impl WriteOptions
{
    #[inline]
    pub fn truncate(mut self) -> Self
    {
        self.0.truncate = true;
        self
    }

    #[inline]
    pub fn create(mut self) -> Self
    {
        self.0.create = true;
        self
    }

    #[inline]
    pub fn read(mut self) -> Self
    {
        self.0.read = true;
        self
    }
}

impl private::Sealed for WriteOptions {}
impl ToOpenOptions for WriteOptions
{
    #[inline]
    fn to_open_options(&self) -> OpenOptions
    {
        self.0.clone()
    }
}

pub struct File
{
    handle: HANDLE,
}

impl File
{
    // @Todo Switch this to AsRef<Path> when it's done
    pub fn open(path: &str, options: impl ToOpenOptions) -> Result<File, io::Error>
    {
        let mut path = WString::from_str(path);
        path.push(0 as char);

        let options = options.to_open_options();

        let (desired_access, share_mode) = match (options.read, options.write)
        {
            (true, false) => (GENERIC_READ, FILE_SHARE_READ),
            (false, true) => (GENERIC_WRITE, 0),
            (true, true) => (GENERIC_READ | GENERIC_WRITE, 0),
            _ => return Err(io::Error::InvalidOpenOptions),
        };

        let creation_disposition = match (
            options.read,
            options.write,
            options.create,
            options.truncate,
        )
        {
            (true, false, _, _) => OPEN_EXISTING,
            (_, true, false, false) => OPEN_EXISTING,
            (_, true, true, false) => OPEN_ALWAYS,
            (_, true, false, true) => TRUNCATE_EXISTING,
            (_, true, true, true) => CREATE_ALWAYS,
            _ => return Err(io::Error::InvalidOpenOptions),
        };

        let handle = unsafe {
            CreateFileW(
                path.as_ptr(),
                desired_access,
                share_mode,
                null_mut(),
                creation_disposition,
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            )
        };

        match handle
        {
            INVALID_HANDLE_VALUE => Err(io::Error::CannotOpen),
            _ => Ok(File { handle }),
        }
    }
}

impl Drop for File
{
    fn drop(&mut self)
    {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

impl io::Seek for File
{
    fn seek(&mut self, pos: io::SeekOrigin) -> io::Result<usize>
    {
        let (method, distance) = match pos
        {
            io::SeekOrigin::Start(distance) => (FILE_BEGIN, distance),
            io::SeekOrigin::End(distance) => (FILE_END, distance),
            io::SeekOrigin::Current(distance) => (FILE_CURRENT, distance),
        };

        let mut new_file_ptr = 0;

        let success =
            unsafe { SetFilePointerEx(self.handle, distance as _, &mut new_file_ptr, method) != 0 };

        match success
        {
            false => Err(io::Error::CannotSeek),
            true => Ok(new_file_ptr as _),
        }
    }
}

impl io::Read for File
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>
    {
        if buf.len() > core::u32::MAX as usize
        {
            return Err(io::Error::BufferTooLarge);
        }

        let mut bytes_read = 0;

        let success = unsafe {
            ReadFile(
                self.handle,
                buf.as_mut_ptr() as _,
                buf.len() as _,
                &mut bytes_read,
                null_mut(),
            ) != 0
        };

        match success
        {
            true => Ok(bytes_read as _),
            false => Err(io::Error::CannotRead),
        }
    }
}

impl io::Write for File
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>
    {
        if buf.len() > core::u32::MAX as usize
        {
            return Err(io::Error::BufferTooLarge);
        }

        let mut bytes_written = 0;

        let success = unsafe {
            WriteFile(
                self.handle,
                buf.as_ptr() as _,
                buf.len() as _,
                &mut bytes_written,
                null_mut(),
            ) != 0
        };

        match success
        {
            true => Ok(bytes_written as _),
            false => Err(io::Error::CannotWrite),
        }
    }

    fn flush(&mut self) -> io::Result<()>
    {
        let success = unsafe { FlushFileBuffers(self.handle) != 0 };

        match success
        {
            true => Ok(()),
            false => Err(io::Error::CannotFlush),
        }
    }
}
