use core::borrow::Borrow;
use core::hash::*;

use crate::alloc::Allocator;
use crate::containers::HashMap;

pub struct HashSet<T, A>
where
    T: Sized + Eq + Hash ,
    A: Allocator + Clone,
{
    map: HashMap<T, (), A>,
}

impl<T, A> HashSet<T, A>
where
    T: Sized + Eq + Hash,
    A: Allocator + Clone,
{
    pub fn new(alloc: A) -> Self
    {
        Self
        {
            map: HashMap::new(alloc),
        }
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.contains(value)
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.remove(value)
    }

    pub fn insert(&mut self, value: T) -> bool
    {
        self.map.insert(value, ())
    }

    pub fn len(&self) -> usize
    {
        self.map.len()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::alloc::Win32HeapAllocator;

    #[test]
    fn contains()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new(&alloc);
        assert!(!set.contains(&5));
        assert!(set.insert(5));
        assert!(set.insert(4));
        assert!(set.insert(6));
        assert!(set.contains(&5));
        assert!(set.contains(&4));
        assert!(set.contains(&6));
        assert!(!set.contains(&0));
        assert!(!set.contains(&9000));
        assert!(!set.contains(&51));
    }

    #[test]
    fn insert()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new(&alloc);

        assert!(set.insert(5));
        assert!(set.insert(4));
        assert!(set.insert(6));
        assert!(!set.insert(5));
        assert!(!set.insert(6));
        assert!(set.insert(7));
    }

    #[test]
    fn len()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new(&alloc);

        assert!(set.len() == 0);
        set.insert(3);
        assert!(set.len() == 1);
        set.insert(1);
        assert!(set.len() == 2);
        set.insert(3);
        assert!(set.len() == 2);
    }

    #[test]
    fn remove()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new(&alloc);

        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert!(!set.remove(&4));
        assert!(set.len() == 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));

        assert!(set.remove(&2));
        assert!(set.len() == 2);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
        assert!(set.contains(&3));

        assert!(!set.remove(&2));
        assert!(set.len() == 2);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
        assert!(set.contains(&3));

        assert!(set.remove(&3));
        assert!(set.len() == 1);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
        assert!(!set.contains(&3));
    }

    #[test]
    fn zst()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashSet::new(&alloc);

        assert!(!set.contains(&()));
        assert!(!set.remove(&()));
        assert!(set.len() == 0);
        assert!(set.insert(()));
        assert!(!set.insert(()));
        assert!(set.contains(&()));
        assert!(set.len() == 1);
        assert!(set.remove(&()));
        assert!(!set.contains(&()));
        assert!(!set.remove(&()));
        assert!(set.len() == 0);
    }
}
