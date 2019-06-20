use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::Array,
};

use core::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
};

pub struct WString<A: Allocator = GlobalAllocator>
{
    buf: Array<u16, A>,
}

impl<A: Allocator> WString<A>
{
    pub fn new_with(alloc: A) -> Self
    {
        Self {
            buf: Array::new_with(alloc),
        }
    }

    pub fn from_str_with(s: &str, alloc: A) -> Self
    {
        let w_iter = s.encode_utf16();

        let mut buf = Array::new_with(alloc);
        buf.reserve(w_iter.size_hint().0);

        for wchar in w_iter
        {
            buf.push(wchar);
        }

        Self { buf }
    }

    pub fn push(&mut self, c: char)
    {
        let len = c.len_utf16();
        self.buf.resize(self.buf.len() + len, 0);

        let start = self.buf.len() - len;
        c.encode_utf16(&mut self.buf[start..]);
    }
}

impl WString<GlobalAllocator>
{
    pub fn new() -> Self
    {
        Self::new_with(GlobalAllocator)
    }

    pub fn from_str(s: &str) -> Self
    {
        Self::from_str_with(s, GlobalAllocator)
    }
}

impl<A: Allocator> AsRef<[u16]> for WString<A>
{
    #[inline]
    fn as_ref(&self) -> &[u16]
    {
        &self.buf
    }
}

impl<A: Allocator> Borrow<[u16]> for WString<A>
{
    #[inline]
    fn borrow(&self) -> &[u16]
    {
        &self.buf
    }
}

impl<A: Allocator> Deref for WString<A>
{
    type Target = [u16];

    #[inline]
    fn deref(&self) -> &[u16]
    {
        &self.buf
    }
}

impl<A: Allocator> DerefMut for WString<A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut [u16]
    {
        &mut self.buf
    }
}
