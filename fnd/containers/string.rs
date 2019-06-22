use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::Array,
};

use core::{
    borrow::Borrow,
    cmp::{Eq, PartialEq},
    fmt, hash,
    ops::{Deref, DerefMut},
    ptr::copy_nonoverlapping,
    str,
};

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

    pub fn push(&mut self, c: char)
    {
        let mut bytes = [0u8; 4];
        c.encode_utf8(&mut bytes);
        self.buf.extend(bytes[0..c.len_utf8()].iter());
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

impl<A: Allocator> fmt::Debug for String<A>
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
    use crate::containers::{HashMap, HashSet};

    #[test]
    fn str()
    {
        let hello = String::from_str("hello");
        let mut world = String::from_str("world");
        world.make_ascii_uppercase();

        assert!(hello.as_str() == "hello");
        assert!(world.as_str() == "WORLD");
        assert!(hello == "hello");
        assert!(world == "WORLD");
    }

    #[test]
    fn set()
    {
        let mut set = HashSet::new();

        assert!(set.insert(String::from_str("hello")));
        assert!(set.insert(String::from_str("HELLO")));
        assert!(!set.insert(String::from_str("hello")));
        assert!(set.len() == 2);
        assert!(set.contains("hello"));
        assert!(!set.contains("world"));
    }

    #[test]
    fn map()
    {
        let mut map = HashMap::new();

        map.insert(String::from_str("Hello"), 42);

        assert!(map.find("Hello") == Some(&42));
        assert!(map.find("world") == None);
    }

    #[test]
    fn push()
    {
        let mut s = String::new();
        s.push('a');
        s.push('é');
        s.push('漢');
        assert_eq!(s, "aé漢");
    }
}
