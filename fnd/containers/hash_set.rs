use core::{borrow::Borrow, hash::*};

use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::HashMap,
};

pub struct HashSet<T, A = GlobalAllocator>
where
    T: Sized + Eq + Hash,
    A: Allocator + Clone,
{
    map: HashMap<T, (), A>,
}

impl<T, A> HashSet<T, A>
where
    T: Sized + Eq + Hash,
    A: Allocator + Clone,
{
    pub fn new_with(alloc: A) -> Self
    {
        Self {
            map: HashMap::new_with(alloc),
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

impl<T> HashSet<T, GlobalAllocator>
where
    T: Sized + Eq + Hash,
{
    pub fn new() -> Self
    {
        Self::new_with(GlobalAllocator)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn contains()
    {
        let mut set = HashSet::new();
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
        let mut set = HashSet::new();

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
        let mut set = HashSet::new();

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
        let mut set = HashSet::new();

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
        let mut set = HashSet::new();

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
