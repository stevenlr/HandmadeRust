use crate::alloc::{Allocator, GlobalAllocator};
use crate::containers::Array;

use core::borrow::Borrow;
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash;
use core::ops::{Deref, DerefMut};
use core::ptr::copy_nonoverlapping;
use core::str;

pub struct String<A: Allocator = GlobalAllocator>
{
    buf: Array<u8, A>,
}

impl<A: Allocator> String<A>
{
    pub fn new_with(alloc: A) -> Self
    {
        Self {
            buf: Array::new_with(alloc),
        }
    }

    pub fn from_str_with(s: &str, alloc: A) -> Self
    {
        let slice = s.as_bytes();
        let mut buf = Array::new_with(alloc);
        buf.resize(slice.len(), 0);

        unsafe {
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

impl String<GlobalAllocator>
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

impl<A: Allocator> AsRef<str> for String<A>
{
    #[inline]
    fn as_ref(&self) -> &str
    {
        self
    }
}

impl<A: Allocator> Borrow<str> for String<A>
{
    #[inline]
    fn borrow(&self) -> &str
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
        unsafe { str::from_utf8_unchecked(&self.buf) }
    }
}

impl<A: Allocator> DerefMut for String<A>
{
    #[inline]
    fn deref_mut(&mut self) -> &mut str
    {
        unsafe { str::from_utf8_unchecked_mut(&mut self.buf) }
    }
}

impl<A: Allocator> fmt::Display for String<A>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<A, T> PartialEq<T> for String<A>
where
    A: Allocator,
    T: AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &T) -> bool
    {
        PartialEq::eq(self.as_str(), other.as_ref())
    }
}

impl<A: Allocator> Eq for String<A> {}

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
    use crate::containers::{HashMap, HashSet};

    #[test]
    fn str()
    {
        let alloc = Win32HeapAllocator::default();
        let hello = String::from_str_with("hello", &alloc);
        let mut world = String::from_str_with("world", &alloc);
        world.make_ascii_uppercase();

        assert!(hello.as_str() == "hello");
        assert!(world.as_str() == "WORLD");
        assert!(hello == "hello");
        assert!(world == "WORLD");
    }

    #[test]
    fn set()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new_with(&alloc);

        assert!(set.insert(String::from_str_with("hello", &alloc)));
        assert!(set.insert(String::from_str_with("HELLO", &alloc)));
        assert!(!set.insert(String::from_str_with("hello", &alloc)));
        assert!(set.len() == 2);
        assert!(set.contains("hello"));
        assert!(!set.contains("world"));
    }

    #[test]
    fn map()
    {
        let alloc = Win32HeapAllocator::default();
        let mut map = HashMap::new_with(&alloc);

        map.insert(String::from_str_with("Hello", &alloc), 42);

        assert!(map.find("Hello") == Some(&42));
        assert!(map.find("world") == None);
    }
}
