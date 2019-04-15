use crate::alloc::Allocator;
use crate::containers::Array;

use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash;
use core::ops::{Deref, DerefMut};
use core::ptr::copy_nonoverlapping;
use core::str;

pub struct String<A: Allocator>
{
    buf: Array<u8, A>,
}

impl<A: Allocator> String<A>
{
    pub fn new(alloc: A) -> Self
    {
        Self
        {
            buf: Array::new(alloc),
        }
    }

    pub fn from_str(s: &str, alloc: A) -> Self
    {
        let slice = s.as_bytes();
        let mut buf = Array::new(alloc);
        buf.resize(slice.len(), 0);

        unsafe
        {
            copy_nonoverlapping(s.as_ptr(), buf.as_mut_ptr(), slice.len());
        }

        Self { buf }
    }

    #[inline]
    pub fn as_str(&self) -> &str
    {
        self
    }
}

impl<A: Allocator> Deref for String<A>
{
    type Target = str;

    #[inline]
    fn deref(&self) -> &str
    {
        unsafe
        {
            str::from_utf8_unchecked(&self.buf)
        }
    }
}

impl<A: Allocator> DerefMut for String<A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut str
    {
        unsafe
        {
            str::from_utf8_unchecked_mut(&mut self.buf)
        }
    }
}

impl<A: Allocator> fmt::Display for String<A>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<A1, A2> PartialEq<String<A2>> for String<A1>
where
    A1: Allocator,
    A2: Allocator,
{
    #[inline]
    fn eq(&self, other: &String<A2>) -> bool
    {
        PartialEq::eq(self.as_str(), other.as_str())
    }
}

impl<A: Allocator> Eq for String<A>
{}

impl<A: Allocator> hash::Hash for String<A>
{
    fn hash<H: hash::Hasher>(&self, h: &mut H)
    {
        hash::Hash::hash(self.as_str(), h);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::Win32HeapAllocator;
    use crate::containers::HashSet;

    #[test]
    fn str()
    {
        let alloc = Win32HeapAllocator::default();
        let hello = String::from_str("hello", &alloc);
        let mut world = String::from_str("world", &alloc);
        world.make_ascii_uppercase();

        assert!(hello.as_str() == "hello");
        assert!(world.as_str() == "WORLD");
    }

    #[test]
    fn set()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new(&alloc);

        assert!(set.insert(String::from_str("hello", &alloc)));
        assert!(set.insert(String::from_str("HELLO", &alloc)));
        assert!(!set.insert(String::from_str("hello", &alloc)));
        assert!(set.len() == 2);
    }
}
