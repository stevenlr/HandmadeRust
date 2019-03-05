use core::borrow::Borrow;
use core::hash::*;
use core::mem::replace;

use crate::alloc::Allocator;
use crate::containers::Array;
use crate::hash::SipHash;

// @Todo Base resize on load factor.

// @Todo Implements Google's awesome array node hash set stuff thingy.

// @Todo Don't clone payloads. Instead use a raw array and read/write/drop manually.

#[derive(Clone)]
enum Bucket<K, V>
where
    K: Sized + Eq + Hash + Clone,
    V: Sized + Clone,
{
    Free,
    Deleted,
    Occupied
    {
        hash: u64,
        key: K,
        value: V,
    },
}

enum FindResult
{
    None,
    Present(usize),
    Free(usize),
}

struct HashMapArray<K, V, A>
where
    K: Sized + Eq + Hash + Clone,
    V: Sized + Clone,
    A: Allocator + Clone,
{
    buckets: Array<Bucket<K, V>, A>,
}

impl<K, V, A> HashMapArray<K, V, A>
where
    K: Sized + Eq + Hash + Clone,
    V: Sized + Clone,
    A: Allocator + Clone,
{
    fn new(alloc: A) -> Self
    {
        Self
        {
            buckets: Array::new(alloc),
        }
    }

    fn find_bucket<Q>(&self, key: &Q, hash: u64) -> FindResult
    where
        Q: Borrow<K>,
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
                Bucket::Occupied{ hash: h, key: ref v, .. } =>
                {
                    if h == hash
                    {
                        if v == key.borrow()
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

pub struct HashMap<K, V, A>
where
    K: Sized + Eq + Hash + Clone,
    V: Sized + Clone,
    A: Allocator + Clone,
{
    alloc: A,
    hash: BuildHasherDefault<SipHash>,
    a: HashMapArray<K, V, A>,
}

impl<K, V, A> HashMap<K, V, A>
where
    K: Sized + Eq + Hash + Clone,
    V: Sized + Clone,
    A: Allocator + Clone,
{
    pub fn new(alloc: A) -> Self
    {
        Self
        {
            alloc: alloc.clone(),
            hash: BuildHasherDefault::default(),
            a: HashMapArray::new(alloc.clone()),
        }
    }

    fn compute_hash<Q>(&self, key: &Q) -> u64
    where
        Q: Borrow<K>,
    {
        let mut hasher = self.hash.build_hasher();
        key.borrow().hash(&mut hasher);
        return hasher.finish();
    }

    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        Q: Borrow<K>,
    {
        let hash = self.compute_hash(key);
        let find = self.a.find_bucket(key, hash);
        match find
        {
            FindResult::Present(_) => true,
            _ => false,
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        Q: Borrow<K>,
    {
        let hash = self.compute_hash(key);
        let find = self.a.find_bucket(key, hash);
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

        let old_array = replace(&mut self.a, HashMapArray::new(self.alloc.clone()));
        self.a.buckets.resize(new_size, Bucket::Free);

        for element in old_array.buckets.iter()
        {
            match element
            {
                Bucket::Occupied{ hash: h, key: ref v, .. } =>
                {
                    let find = self.a.find_bucket(v, *h);
                    match find
                    {
                        FindResult::Free(index) =>
                        {
                            self.a.buckets[index] = element.clone();
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

    pub fn insert(&mut self, key: K, value: V) -> bool
    {
        let hash = self.compute_hash(&key);
        let find = self.a.find_bucket(&key, hash);

        match find
        {
            FindResult::Present(index) =>
            {
                self.a.buckets[index] = Bucket::Occupied
                {
                    hash,
                    key,
                    value,
                };
                return false;
            },
            FindResult::Free(index) =>
            {
                self.a.buckets[index] = Bucket::Occupied
                {
                    hash,
                    key,
                    value,
                };
                return true;
            },
            FindResult::None => {},
        }

        self.grow();

        let find = self.a.find_bucket(&key, hash);
        match find
        {
            FindResult::Free(index) =>
            {
                self.a.buckets[index] = Bucket::Occupied
                {
                    hash,
                    key,
                    value,
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

    pub fn find<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Borrow<K>,
    {
        let hash = self.compute_hash(key);
        let find = self.a.find_bucket(key, hash);
        match find
        {
            FindResult::Present(index) =>
            {
                match self.a.buckets[index]
                {
                    Bucket::Occupied{ value: ref v, .. } =>
                    {
                        Some(v)
                    },
                    _ => None,
                }
            }
            _ => None,
        }
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
        let mut set = HashMap::new(&alloc);
        assert!(!set.contains(&5));
        assert!(set.insert(5, 4));
        assert!(set.insert(4, 4));
        assert!(set.insert(6, 4));
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
        let mut set = HashMap::new(&alloc);

        assert!(set.insert(5, 1));
        assert!(set.insert(4, 2));
        assert!(set.insert(6, 3));
        assert!(!set.insert(5, 4));
        assert!(!set.insert(6, 5));
        assert!(set.insert(7, 6));
    }

    #[test]
    fn len()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashMap::new(&alloc);

        assert!(set.len() == 0);
        set.insert(3, 0);
        assert!(set.len() == 1);
        set.insert(1, 0);
        assert!(set.len() == 2);
        set.insert(3, 0);
        assert!(set.len() == 2);
    }

    #[test]
    fn remove()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashMap::new(&alloc);

        set.insert(1, 0);
        set.insert(2, 0);
        set.insert(3, 0);

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
        let mut set = HashMap::new(&alloc);

        assert!(!set.contains(&()));
        assert!(!set.remove(&()));
        assert!(set.len() == 0);
        assert!(set.insert((), ()));
        assert!(!set.insert((), ()));
        assert!(set.contains(&()));
        assert!(set.len() == 1);
        assert!(set.remove(&()));
        assert!(!set.contains(&()));
        assert!(!set.remove(&()));
        assert!(set.len() == 0);
    }

    #[test]
    fn find()
    {
        let alloc = Win32HeapAllocator::default();
        let mut set = HashMap::new(&alloc);

        set.insert(0, 0);
        set.insert(1, 1);
        set.insert(2, 0);
        set.insert(3, 1);

        assert!(set.find(&0) == Some(&0));
        assert!(set.find(&1) == Some(&1));
        assert!(set.find(&2) == Some(&0));
        assert!(set.find(&3) == Some(&1));
        assert!(set.find(&5) == None);

        set.insert(1, 2);

        assert!(set.find(&0) == Some(&0));
        assert!(set.find(&1) == Some(&2));
        assert!(set.find(&2) == Some(&0));
        assert!(set.find(&3) == Some(&1));
    }
}
