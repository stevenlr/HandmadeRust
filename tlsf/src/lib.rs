#![no_std]
#![allow(dead_code)] // @Todo Remove this

use core::{
    mem::{align_of, size_of},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

use fnd::containers::Array;

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unsupported pointer width");

const L: usize = 5;
const MIN_FL: usize = 5; // 32 o
const MAX_FL: usize = 31; // 4 Go

const MIN_SIZE: usize = 1 << MIN_FL;
const MAX_SIZE: usize = (1 << (MAX_FL + 1)) - 1;

const SL_COUNT: usize = 1 << L;
const FL_COUNT: usize = MAX_FL - MIN_FL;

pub struct Tlsf
{
    fl_bitmap: usize,
    sl_bitmap: [usize; FL_COUNT],
    buckets: Array<FreeBlockHeader>,
}

#[derive(Clone, Copy)]
struct Bucket
{
    fl: usize,
    sl: usize,
}

#[inline]
fn find_last_set(r: usize) -> usize
{
    size_of::<usize>() * 8 - r.leading_zeros() as usize - 1
}

fn mapping_insert(r: usize) -> Bucket
{
    let fl = find_last_set(r);
    let sl = (r >> (fl - L)) - SL_COUNT;

    return Bucket { fl, sl };
}

fn mapping_search(mut r: usize) -> (usize, Bucket)
{
    r += (1 << (find_last_set(r) - L)) - 1;

    let fl = find_last_set(r);
    let sl = (r >> (fl - L)) - SL_COUNT;

    return (r, Bucket { fl, sl });
}

#[repr(C)]
struct FreeBlockHeader
{
    used_header: BlockHeader,
    next_free: Option<NonNull<FreeBlockHeader>>, // @Todo Use sentinel to remove null checks
    prev_free: Option<NonNull<FreeBlockHeader>>,
}

#[repr(C)]
struct BlockHeader
{
    size_flags: usize,
    prev_phys_block: Option<NonNull<BlockHeader>>,
}

const LAST_BIT: usize = 0b0001;
const FREE_BIT: usize = 0b0010;
const ALL_FLAGS: usize = LAST_BIT | FREE_BIT;

impl Default for FreeBlockHeader
{
    fn default() -> Self
    {
        FreeBlockHeader {
            used_header: BlockHeader::default(),
            next_free: None,
            prev_free: None,
        }
    }
}

impl Default for BlockHeader
{
    fn default() -> Self
    {
        BlockHeader {
            size_flags: 0,
            prev_phys_block: None,
        }
    }
}

impl Deref for FreeBlockHeader
{
    type Target = BlockHeader;

    fn deref(&self) -> &BlockHeader
    {
        &self.used_header
    }
}

impl DerefMut for FreeBlockHeader
{
    fn deref_mut(&mut self) -> &mut BlockHeader
    {
        &mut self.used_header
    }
}

impl BlockHeader
{
    #[inline]
    fn flags(&self) -> usize
    {
        self.size_flags & ALL_FLAGS
    }

    #[inline]
    fn is_free(&self) -> bool
    {
        (self.size_flags & FREE_BIT) != 0
    }

    #[inline]
    fn is_last(&self) -> bool
    {
        (self.size_flags & LAST_BIT) != 0
    }

    #[inline]
    fn size(&self) -> usize
    {
        self.size_flags & (!ALL_FLAGS)
    }

    fn set_size(&mut self, size: usize)
    {
        assert!(size % align_of::<FreeBlockHeader>() == 0);
        self.size_flags = size | self.flags();
    }

    #[inline]
    fn set_flag(&mut self, flag: usize, set: bool)
    {
        let flags = if set
        {
            (self.flags() | flag)
        }
        else
        {
            (self.flags() & (!flag))
        } & ALL_FLAGS;

        self.size_flags = self.size() | flags;
    }

    fn set_last(&mut self, is_last: bool)
    {
        self.set_flag(LAST_BIT, is_last);
    }

    fn set_free(&mut self, is_free: bool)
    {
        self.set_flag(FREE_BIT, is_free);
    }

    fn bucket(&self) -> Bucket
    {
        mapping_insert(self.size())
    }

    fn next_phys(&self) -> Option<NonNull<BlockHeader>>
    {
        if self.is_last()
        {
            return None;
        }

        let ptr = NonNull::from(self).cast::<u8>().as_ptr();
        let ptr = unsafe { ptr.offset(self.size() as isize) } as *mut BlockHeader;
        NonNull::new(ptr)
    }

    fn prev_phys(&self) -> Option<NonNull<BlockHeader>>
    {
        self.prev_phys_block
    }
}

impl FreeBlockHeader
{
    fn insert_after(&mut self, mut block_ptr: NonNull<FreeBlockHeader>)
    {
        let free_block = unsafe { block_ptr.as_mut() };

        free_block.next_free = self.next_free;
        free_block.prev_free = Some(self.into());

        if let Some(next_ptr) = &mut self.next_free
        {
            let next_block = unsafe { next_ptr.as_mut() };
            next_block.prev_free = Some(block_ptr);
        }

        self.next_free = Some(block_ptr);
    }

    fn detach(&mut self)
    {
        if let Some(ptr) = &mut self.next_free
        {
            let block = unsafe { ptr.as_mut() };
            block.prev_free = self.prev_free;
        }

        if let Some(ptr) = &mut self.prev_free
        {
            let block = unsafe { ptr.as_mut() };
            block.next_free = self.next_free;
        }

        self.next_free = None;
        self.prev_free = None;
    }
}

impl Tlsf
{
    pub fn new() -> Self
    {
        let mut buckets = Array::new();
        buckets.resize_default(FL_COUNT * SL_COUNT);

        Self {
            fl_bitmap: 0,
            sl_bitmap: [0; FL_COUNT],
            buckets,
        }
    }

    fn add_physical_block(&mut self, ptr: NonNull<u8>, size: usize)
    {
        let mut ptr = ptr.cast::<FreeBlockHeader>();

        assert!(
            (ptr.as_ptr() as usize) % align_of::<FreeBlockHeader>() == 0,
            "Physical block alignment"
        );

        assert!(size >= MIN_SIZE && size <= MAX_SIZE, "Physical block size");

        let free_block = unsafe {
            ptr::write(ptr.as_ptr(), FreeBlockHeader::default());
            ptr.as_mut()
        };

        free_block.set_size(size);
        free_block.set_last(true);
        free_block.set_free(true);

        let bucket = free_block.bucket();
        self.insert_free(ptr, bucket);
    }

    fn bucket_list_head_mut(&mut self, bucket: Bucket) -> &mut FreeBlockHeader
    {
        debug_assert!(bucket.fl >= MIN_FL && bucket.fl <= MAX_FL && bucket.sl <= SL_COUNT);
        return &mut self.buckets[(bucket.fl - MIN_FL) * FL_COUNT + bucket.sl];
    }

    fn insert_free(&mut self, block_ptr: NonNull<FreeBlockHeader>, bucket: Bucket)
    {
        self.bucket_list_head_mut(bucket).insert_after(block_ptr);

        let fl = bucket.fl - MIN_FL;
        self.fl_bitmap |= 1 << fl;
        self.sl_bitmap[fl] |= 1 << bucket.sl;
    }

    fn remove_free(&mut self, mut block_ptr: NonNull<FreeBlockHeader>, bucket: Bucket)
    {
        unsafe { block_ptr.as_mut() }.detach();

        if self.bucket_list_head_mut(bucket).next_free.is_none()
        {
            let fl = bucket.fl - MIN_FL;
            self.sl_bitmap[fl] &= !(1 << bucket.sl);

            if self.sl_bitmap[fl] == 0
            {
                self.fl_bitmap &= !(1 << fl);
            }
        }
    }

    // alloc @Todo
    //
    // adjust size?
    // mapping search
    // find block
    // if found
    //      remove found
    //      if found size > required
    //          split
    //          mapping insert remaining
    //          insert remaining
    // else
    //      add phys block, restart
    //      restart
    // return found

    // free @Todo
    //
    // mark free
    // mapping insert
    // insert
    // merge previous and next

    fn merge_with_next_phys(&mut self, mut this_ptr: NonNull<FreeBlockHeader>)
    {
        let this = unsafe { this_ptr.as_mut() };
        let next_ptr = this.next_phys();

        if let Some(mut next_ptr) = next_ptr
        {
            let next = unsafe { next_ptr.as_mut() };

            if next.is_free()
            {
                let mut next_ptr = next_ptr.cast::<FreeBlockHeader>();
                let next = unsafe { next_ptr.as_mut() };

                let this_bucket = this.bucket();
                let next_bucket = next.bucket();

                self.remove_free(this.into(), this_bucket);
                self.remove_free(next.into(), next_bucket);

                let new_size = this.size() + next.size();
                this.set_size(new_size);
                this.set_last(next.is_last());

                self.insert_free(this.into(), this.bucket());
            }
        }
    }

    fn merge_with_neighbors_phys(&mut self, mut this_ptr: NonNull<FreeBlockHeader>)
    {
        self.merge_with_next_phys(this_ptr);

        if let Some(mut prev_ptr) = unsafe { this_ptr.as_mut() }.prev_phys()
        {
            let prev = unsafe { prev_ptr.as_mut() };

            if prev.is_free()
            {
                let prev_ptr = prev_ptr.cast::<FreeBlockHeader>();
                self.merge_with_next_phys(prev_ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn headers_layout()
    {
        assert_eq!(size_of::<BlockHeader>(), size_of::<usize>() * 2);
        assert_eq!(align_of::<BlockHeader>(), size_of::<usize>());
        assert_eq!(size_of::<FreeBlockHeader>(), size_of::<usize>() * 4);
        assert_eq!(align_of::<FreeBlockHeader>(), size_of::<usize>());
    }
}
