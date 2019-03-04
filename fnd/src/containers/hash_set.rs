use core::borrow::Borrow;
use core::hash::*;
use core::mem::replace;

use crate::alloc::Allocator;
use crate::containers::Array;
use crate::hash::SipHash;

#[derive(Clone)]
enum Bucket<T: Sized + Eq + Hash + Clone>
{
    Free,
    Deleted,
    Occupied
    {
        hash: u64,
        value: T,
    },
}

enum FindResult
{
    None,
    Present(usize),
    Free(usize),
}

struct HashSetArray<T, A>
where
    T: Sized + Eq + Hash + Clone,
    A: Allocator + Clone,
{
    buckets: Array<Bucket<T>, A>,
}

impl<T, A> HashSetArray<T, A>
where
    T: Sized + Eq + Hash + Clone,
    A: Allocator + Clone,
{
    fn new(alloc: A) -> Self
    {
        Self
        {
            buckets: Array::new(alloc),
        }
    }

    fn find_bucket<Q>(&self, value: &Q, hash: u64) -> FindResult
    where
        Q: Borrow<T>,
    {
        if self.buckets.is_empty()
        {
            return FindResult::None;
        }

        let start_index = (hash % self.buckets.len() as u64) as usize;
        let mut index = start_index;

        let mut acceptable_for_insert = FindResult::None;

        loop
        {
            match self.buckets[index]
            {
                Bucket::Free => return FindResult::Free(index),
                Bucket::Occupied{ hash: h, value: ref v } =>
                {
                    if h == hash
                    {
                        if v == value.borrow()
                        {
                            return FindResult::Present(index);
                        }
                    }
                },
                Bucket::Deleted =>
                {
                    if let FindResult::None = acceptable_for_insert
                    {
                        acceptable_for_insert = FindResult::Free(index);
                    }
                },
            }

            let new_index = (index + 1) % self.buckets.len();
            if new_index == start_index
            {
                break;
            }

            index = new_index;
        }

        return acceptable_for_insert;
    }
}

pub struct HashSet<T, A>
where
    T: Sized + Eq + Hash + Clone,
    A: Allocator + Clone,
{
    alloc: A,
    hash: BuildHasherDefault<SipHash>,
    a: HashSetArray<T, A>,
}

impl<T, A> HashSet<T, A>
where
    T: Sized + Eq + Hash + Clone,
    A: Allocator + Clone,
{
    pub fn new(alloc: A) -> Self
    {
        Self
        {
            alloc: alloc.clone(),
            hash: BuildHasherDefault::default(),
            a: HashSetArray::new(alloc.clone()),
        }
    }

    fn compute_hash<Q>(&self, value: &Q) -> u64
    where
        Q: Borrow<T>,
    {
        let mut hasher = self.hash.build_hasher();
        value.borrow().hash(&mut hasher);
        return hasher.finish();
    }


    pub fn contains<Q>(&self, value: &Q) -> bool
    where
        Q: Borrow<T>,
    {
        let hash = self.compute_hash(value);
        let find = self.a.find_bucket(value, hash);
        match find
        {
            FindResult::Present(_) => true,
            _ => false,
        }
    }

    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: Borrow<T>,
    {
        let hash = self.compute_hash(value);
        let find = self.a.find_bucket(value, hash);
        match find
        {
            FindResult::Present(index) =>
            {
                self.a.buckets[index] = Bucket::Deleted;
                return true;
            },
            _ => false,
        }
    }

    fn grow(&mut self)
    {
        let new_size = if self.a.buckets.len() == 0
        {
            1
        }
        else
        {
            self.a.buckets.len() * 2
        };

        let old_array = replace(&mut self.a, HashSetArray::new(self.alloc.clone()));
        self.a.buckets.resize(new_size, Bucket::Free);

        for element in old_array.buckets.iter()
        {
            match element
            {
                Bucket::Occupied{ hash: h, value: ref v } =>
                {
                    let find = self.a.find_bucket(v, *h);
                    match find
                    {
                        FindResult::Free(index) =>
                        {
                            // @Todo We shouldn't have to clone these...
                            self.a.buckets[index] = Bucket::Occupied
                            {
                                hash: *h,
                                value: v.clone(),
                            };
                        },
                        FindResult::None | FindResult::Present(_) =>
                        {
                            panic!("New hash set not large enough");
                        },
                    }
                },
                _ => {},
            }
        }
    }

    pub fn insert(&mut self, value: T) -> bool
    {
        let hash = self.compute_hash(&value);
        let find = self.a.find_bucket(&value, hash);

        match find
        {
            FindResult::Present(_) => return false,
            FindResult::Free(index) =>
            {
                self.a.buckets[index] = Bucket::Occupied
                {
                    hash: hash,
                    value: value,
                };
                return true;
            },
            FindResult::None => {},
        }

        self.grow();

        let find = self.a.find_bucket(&value, hash);
        match find
        {
            FindResult::Free(index) =>
            {
                self.a.buckets[index] = Bucket::Occupied
                {
                    hash: hash,
                    value: value,
                };
                return true;
            },
            _ =>
            {
                panic!("Error inserting element");
            },
        }
    }

    pub fn len(&self) -> usize
    {
        self.a.buckets.iter()
            .filter(|x|
            {
                match x
                {
                    Bucket::Occupied{..} => true,
                    _ => false,
                }
            })
            .count()
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
