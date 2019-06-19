use super::io::*;

use win32::{
    kernel32::{CloseHandle, CreateFileW, ReadFile, SetFilePointerEx, WriteFile},
    *,
};

mod private
{
    pub trait Sealed
    {
    }
}

#[derive(Default)]
pub struct OpenOptions
{
    read: bool,
    write: bool,
    truncate: bool,
    create: bool,
}

pub trait AsOpenOptions: private::Sealed
{
    fn as_open_options(&self) -> &OpenOptions;
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
impl AsOpenOptions for ReadOptions
{
    #[inline]
    fn as_open_options(&self) -> &OpenOptions
    {
        &self.0
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
impl AsOpenOptions for WriteOptions
{
    #[inline]
    fn as_open_options(&self) -> &OpenOptions
    {
        &self.0
    }
}

pub struct File
{
    handle: HANDLE,
}

impl File
{
    // @Todo Switch this to AsRef<Path> when, it's done
    pub fn open(path: &str, options: impl AsOpenOptions)
    {
        let options = options.as_open_options();
    }
}
