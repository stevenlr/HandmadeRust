use core::{
    borrow::Borrow,
    hash::*,
    mem::{replace, swap},
};

use crate::{
    alloc::{Allocator, GlobalAllocator},
    containers::Array,
    hash::SipHash,
};

// @Todo Base resize on load factor.

struct Bucket<K, V>
where
    K: Sized + Eq + Hash,
    V: Sized,
{
    hash: u64,
    key: K,
    value: V,
}

#[derive(PartialEq, Clone, Copy)]
struct ShortBucket(u8);

impl Default for ShortBucket
{
    #[inline]
    fn default() -> Self
    {
        Self(Self::FREE)
    }
}

impl ShortBucket
{
    const FREE: u8 = 1;
    const DELETED: u8 = 2;

    #[inline]
    pub fn occupied(hash: u64) -> Self
    {
        Self(0x80 | (hash & 0x7f) as u8)
    }

    #[inline]
    pub fn free() -> Self
    {
        Self(Self::FREE)
    }

    #[inline]
    pub fn deleted() -> Self
    {
        Self(Self::DELETED)
    }

    #[inline]
    pub fn is_occupied(&self) -> bool
    {
        self.0 & 0x80 != 0
    }

    #[inline]
    pub fn is_free(&self) -> bool
    {
        self.0 == Self::FREE
    }

    #[inline]
    pub fn is_deleted(&self) -> bool
    {
        self.0 == Self::DELETED
    }
}

enum FindResult
{
    None,
    Present(usize),
    Free(usize),
}

struct HashMapInner<K, V, A>
where
    K: Sized + Eq + Hash,
    V: Sized,
    A: Allocator + Clone,
{
    shortb: Array<ShortBucket, A>,
    buckets: Array<Option<Bucket<K, V>>, A>,
}

impl<K, V, A> HashMapInner<K, V, A>
where
    K: Sized + Eq + Hash,
    V: Sized,
    A: Allocator + Clone,
{
    fn new_with(alloc: A) -> Self
    {
        Self {
            shortb: Array::new_with(alloc.clone()),
            buckets: Array::new_with(alloc.clone()),
        }
    }

    pub fn grow_and_insert(&mut self, old: &mut HashMapInner<K, V, A>)
    {
        let new_size = if old.buckets.len() == 0
        {
            16
        }
        else
        {
            old.buckets.len() * 2
        };

        self.buckets.resize_with(new_size, || None);
        self.shortb.resize(new_size, ShortBucket::free());

        for mut element in old.buckets.iter_mut()
        {
            match element
            {
                Some(Bucket {
                    hash: h,
                    key: ref k,
                    ..
                }) =>
                {
                    let find = self.find_bucket(k, *h);
                    match find
                    {
                        FindResult::Free(new_index) =>
                        {
                            self.shortb[new_index] = ShortBucket::occupied(*h);
                            swap(&mut self.buckets[new_index], &mut element);
                        }
                        FindResult::None | FindResult::Present(_) =>
                        {
                            panic!("New hash map not large enough");
                        }
                    }
                }
                _ =>
                {}
            }
        }
    }

    pub fn contains<Q>(&self, hash: u64, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let find = self.find_bucket(key, hash);
        match find
        {
            FindResult::Present(_) => true,
            _ => false,
        }
    }

    pub fn remove<Q>(&mut self, hash: u64, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let find = self.find_bucket(key, hash);
        match find
        {
            FindResult::Present(index) =>
            {
                self.shortb[index] = ShortBucket::deleted();
                self.buckets[index] = None;
                return true;
            }
            _ => false,
        }
    }

    pub fn find<Q>(&self, hash: u64, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let find = self.find_bucket(key, hash);
        match find
        {
            FindResult::Present(index) => match self.buckets[index]
            {
                Some(Bucket { value: ref v, .. }) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn find_mut<Q>(&mut self, hash: u64, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        let find = self.find_bucket(key, hash);
        match find
        {
            FindResult::Present(index) => match self.buckets[index]
            {
                Some(Bucket {
                    value: ref mut v, ..
                }) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn len(&self) -> usize
    {
        self.shortb.iter().filter(|x| x.is_occupied()).count()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K>
    {
        self.shortb
            .iter()
            .zip(self.buckets.iter())
            .filter(|(x, _)| x.is_occupied())
            .map(|(_, y)| &y.as_ref().unwrap().key)
    }

    pub fn keys_values(&self) -> impl Iterator<Item = (&K, &V)>
    {
        self.shortb
            .iter()
            .zip(self.buckets.iter())
            .filter(|(x, _)| x.is_occupied())
            .map(|(_, y)| {
                let r = y.as_ref().unwrap();
                (&r.key, &r.value)
            })
    }

    fn find_bucket<Q>(&self, key: &Q, hash: u64) -> FindResult
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized,
    {
        if self.buckets.is_empty()
        {
            return FindResult::None;
        }

        let start_index = (hash % self.buckets.len() as u64) as usize;
        let mut index = start_index;
        let mut acceptable_for_insert = FindResult::None;

        let shortb_reference = ShortBucket::occupied(hash);

        loop
        {
            let shortb = &self.shortb[index];

            if *shortb == shortb_reference
            {
                let bucket = self.buckets[index].as_ref().unwrap();
                if bucket.hash == hash && bucket.key.borrow() == key
                {
                    return FindResult::Present(index);
                }
            }
            else if shortb.is_free()
            {
                if let FindResult::None = acceptable_for_insert
                {
                    return FindResult::Free(index);
                }
                else
                {
                    return acceptable_for_insert;
                }
            }
            else if shortb.is_deleted()
            {
                if let FindResult::None = acceptable_for_insert
                {
                    acceptable_for_insert = FindResult::Free(index);
                }
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

pub struct HashMap<K, V, A = GlobalAllocator>
where
    K: Sized + Eq + Hash,
    V: Sized,
    A: Allocator + Clone,
{
    alloc: A,
    hash: BuildHasherDefault<SipHash>,
    inner: HashMapInner<K, V, A>,
}

impl<K, V, A> HashMap<K, V, A>
where
    K: Sized + Eq + Hash,
    V: Sized,
    A: Allocator + Clone,
{
    pub fn new_with(alloc: A) -> Self
    {
        Self {
            alloc: alloc.clone(),
            hash: BuildHasherDefault::default(),
            inner: HashMapInner::new_with(alloc.clone()),
        }
    }

    fn compute_hash<Q>(&self, key: &Q) -> u64
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = self.hash.build_hasher();
        key.hash(&mut hasher);
        return hasher.finish();
    }

    #[inline]
    pub fn contains<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.compute_hash(key);
        return self.inner.contains(hash, key);
    }

    #[inline]
    pub fn remove<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.compute_hash(key);
        return self.inner.remove(hash, key);
    }

    fn grow(&mut self)
    {
        let mut old = replace(&mut self.inner, HashMapInner::new_with(self.alloc.clone()));
        self.inner.grow_and_insert(&mut old);
    }

    pub fn insert(&mut self, key: K, value: V) -> bool
    {
        let hash = self.compute_hash(&key);
        let find = self.inner.find_bucket(&key, hash);

        match find
        {
            FindResult::Present(index) =>
            {
                self.inner.buckets[index] = Some(Bucket { hash, key, value });
                return false;
            }
            FindResult::Free(index) =>
            {
                self.inner.buckets[index] = Some(Bucket { hash, key, value });
                self.inner.shortb[index] = ShortBucket::occupied(hash);
                return true;
            }
            FindResult::None =>
            {}
        }

        self.grow();

        let find = self.inner.find_bucket(&key, hash);
        match find
        {
            FindResult::Free(index) =>
            {
                self.inner.buckets[index] = Some(Bucket { hash, key, value });
                self.inner.shortb[index] = ShortBucket::occupied(hash);
                return true;
            }
            _ =>
            {
                panic!("Error inserting element");
            }
        }
    }

    #[inline]
    pub fn len(&self) -> usize
    {
        self.inner.len()
    }

    #[inline]
    pub fn keys(&self) -> impl Iterator<Item = &K>
    {
        self.inner.keys()
    }

    #[inline]
    pub fn keys_values(&self) -> impl Iterator<Item = (&K, &V)>
    {
        self.inner.keys_values()
    }

    pub fn find<'a, Q>(&'a self, key: &Q) -> Option<&'a V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.compute_hash(key);
        return self.inner.find(hash, key);
    }

    pub fn find_mut<'a, Q>(&'a mut self, key: &Q) -> Option<&'a mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.compute_hash(key);
        return self.inner.find_mut(hash, key);
    }
}

impl<K, V> HashMap<K, V, GlobalAllocator>
where
    K: Sized + Eq + Hash,
    V: Sized,
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
    use crate::alloc::SystemAllocator;

    #[test]
    fn contains()
    {
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);
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
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

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
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

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
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

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
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

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
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

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

    #[test]
    fn iter_keys()
    {
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

        set.insert(0, 0);
        set.insert(1, 1);
        set.insert(4, 0);
        set.insert(5, 1);

        let mut visited = [0, 0, 0, 0, 0, 0];

        set.keys().for_each(|i| visited[*i as usize] += 1);

        assert_eq!(visited[0], 1);
        assert_eq!(visited[1], 1);
        assert_eq!(visited[2], 0);
        assert_eq!(visited[3], 0);
        assert_eq!(visited[4], 1);
        assert_eq!(visited[5], 1);
    }

    #[test]
    fn iter_keys_values()
    {
        let alloc = SystemAllocator::default();
        let mut set = HashMap::new_with(&alloc);

        set.insert(0, 7);
        set.insert(1, 8);
        set.insert(4, 9);
        set.insert(5, 10);

        let mut visited = [0, 0, 0, 0, 0, 0];

        set.keys_values().for_each(|(i, j)| {
            visited[*i as usize] += 1;
            match *i
            {
                0 => assert_eq!(*j, 7),
                1 => assert_eq!(*j, 8),
                4 => assert_eq!(*j, 9),
                5 => assert_eq!(*j, 10),
                _ => assert!(false),
            }
        });

        assert_eq!(visited[0], 1);
        assert_eq!(visited[1], 1);
        assert_eq!(visited[2], 0);
        assert_eq!(visited[3], 0);
        assert_eq!(visited[4], 1);
        assert_eq!(visited[5], 1);
    }
}
