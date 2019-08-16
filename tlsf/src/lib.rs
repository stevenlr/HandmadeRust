#![no_std]

use core::{
    ffi::c_void,
    mem::{align_of, size_of},
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr::{self, NonNull},
};

use fnd::{
    alloc::{Allocator, Layout, SystemAllocator},
    containers::Array,
    Unq,
};

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("Unsupported pointer width");

const L: usize = 5;
const MIN_FL: usize = 5; // 32 o
const MAX_FL: usize = 31; // 4 Go
const PHYS_BLOCK_SIZE: usize = 256 * 1024 * 1024;

const MIN_SIZE: usize = 1 << MIN_FL;
const MAX_SIZE: usize = (1 << (MAX_FL + 1)) - 1;

const SL_COUNT: usize = 1 << L;
const FL_COUNT: usize = MAX_FL - MIN_FL;
const BUCKET_COUNT: usize = SL_COUNT * FL_COUNT;

pub struct Tlsf<A: Allocator = SystemAllocator>
{
    fl_bitmap: usize,
    sl_bitmap: [usize; FL_COUNT],
    buckets:   Pin<Unq<[FreeBlockHeader; BUCKET_COUNT], A>>,
    allocated: Array<NonNull<c_void>, A>,
    alloc:     A,
}

unsafe impl<A: Allocator> Send for Tlsf<A> {}

#[derive(Clone, Copy)]
struct Bucket
{
    fl: usize,
    sl: usize,
}

#[inline]
fn find_last_set(r: usize) -> Option<usize>
{
    if r == 0
    {
        None
    }
    else
    {
        Some(size_of::<usize>() * 8 - r.leading_zeros() as usize - 1)
    }
}

#[inline]
fn find_first_set(r: usize) -> Option<usize>
{
    if r == 0
    {
        None
    }
    else
    {
        Some(r.trailing_zeros() as usize)
    }
}

fn mapping_insert(r: usize) -> Option<Bucket>
{
    let fl = find_last_set(r)?;
    let sl = (r >> (fl - L)) - SL_COUNT;

    return Some(Bucket { fl, sl });
}

fn mapping_search(mut r: usize) -> Option<(usize, Bucket)>
{
    r += (1 << (find_last_set(r)? - L)) - 1;

    let fl = find_last_set(r)?;
    let sl = (r >> (fl - L)) - SL_COUNT;

    return Some((r, Bucket { fl, sl }));
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FreeBlockHeader
{
    used_header: BlockHeader,
    next_free:   Option<NonNull<FreeBlockHeader>>,
    prev_free:   Option<NonNull<FreeBlockHeader>>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BlockHeader
{
    size_flags:      usize,
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
            next_free:   None,
            prev_free:   None,
        }
    }
}

impl Default for BlockHeader
{
    fn default() -> Self
    {
        BlockHeader {
            size_flags:      0,
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

    fn bucket(&self) -> Option<Bucket>
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

    fn update_prev_of_next_phys(&self)
    {
        if let Some(mut ptr) = self.next_phys()
        {
            let next = unsafe { ptr.as_mut() };
            next.prev_phys_block = Some(self.into());
        }
    }

    fn payload_ptr(&self) -> NonNull<u8>
    {
        unsafe { NonNull::new_unchecked((&*self as *const BlockHeader).offset(1) as *mut u8) }
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

impl Tlsf<SystemAllocator>
{
    pub fn new() -> Self
    {
        Self::new_with(SystemAllocator {})
    }
}

impl<A: Allocator + Clone> Tlsf<A>
{
    pub fn new_with(alloc: A) -> Self
    {
        let buckets = Unq::pin_with([FreeBlockHeader::default(); BUCKET_COUNT], alloc.clone());

        Self {
            fl_bitmap: 0,
            sl_bitmap: [0; FL_COUNT],
            buckets,
            allocated: Array::new_with(alloc.clone()),
            alloc,
        }
    }

    fn add_new_physical_block(&mut self, min_size: usize) -> Option<()>
    {
        let size = min_size.max(PHYS_BLOCK_SIZE);

        let mut layout = Layout::from_type::<BlockHeader>();
        layout.size = size;

        let allocated = unsafe { self.alloc.alloc_aligned(layout)? };

        self.add_physical_block(allocated, size);
        Some(())
    }

    fn add_physical_block(&mut self, ptr: NonNull<c_void>, size: usize)
    {
        self.allocated.push(ptr);

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

        let bucket = free_block.bucket().unwrap();
        self.insert_free(ptr, bucket);
    }

    fn bucket_list_head(&self, bucket: Bucket) -> &FreeBlockHeader
    {
        debug_assert!(bucket.fl >= MIN_FL && bucket.fl <= MAX_FL && bucket.sl <= SL_COUNT);
        return &self.buckets[(bucket.fl - MIN_FL) * FL_COUNT + bucket.sl];
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

    fn find_suitable_block(&self, bucket: Bucket) -> Option<NonNull<FreeBlockHeader>>
    {
        let fl = bucket.fl - MIN_FL;
        let sl = bucket.sl;

        let tmp_bitmap = self.sl_bitmap[fl] & ((!0usize) << sl);

        let (fl, sl) = if tmp_bitmap != 0
        {
            (fl, find_first_set(tmp_bitmap)?)
        }
        else
        {
            let tmp_bitmap = self.fl_bitmap & ((!0usize) << (fl + 1));
            let fl = find_first_set(tmp_bitmap)?;
            (fl, find_first_set(self.sl_bitmap[fl])?)
        };

        let bucket = Bucket {
            fl: fl + MIN_FL,
            sl,
        };
        return self.bucket_list_head(bucket).next_free;
    }

    fn alloc_inner(&mut self, src_size: usize) -> Option<NonNull<u8>>
    {
        const SIZE_ALIGN: usize = size_of::<FreeBlockHeader>();

        let size = (src_size + size_of::<BlockHeader>()).max(MIN_SIZE);
        let size = (size + SIZE_ALIGN - 1) & !(SIZE_ALIGN - 1);

        debug_assert!(size % SIZE_ALIGN == 0);

        if size > MAX_SIZE
        {
            return None;
        }

        let (_, bucket) = mapping_search(size)?;
        let block_ptr = self.find_suitable_block(bucket);

        if let Some(mut block_ptr) = block_ptr
        {
            self.maybe_split_phys(block_ptr, size);

            let block = unsafe { block_ptr.as_mut() };
            self.remove_free(block.into(), block.bucket()?);
            block.set_free(false);

            return Some(block.payload_ptr());
        }
        else
        {
            self.add_new_physical_block(size * 2)?;
            return self.alloc_inner(src_size);
        }
    }

    fn dealloc_inner(&mut self, ptr: NonNull<u8>)
    {
        let mut ptr: NonNull<FreeBlockHeader> =
            unsafe { NonNull::new_unchecked(ptr.cast::<BlockHeader>().as_ptr().offset(-1)) }
                .cast::<FreeBlockHeader>();

        let block = unsafe { ptr.as_mut() };
        block.set_free(true);

        self.insert_free(block.into(), block.bucket().unwrap());
        self.maybe_merge_with_neighbors_phys(block.into());
    }

    fn maybe_split_phys(&mut self, mut this_ptr: NonNull<FreeBlockHeader>, size: usize)
    {
        let this = unsafe { this_ptr.as_mut() };

        if this.size() < size + MIN_SIZE
        {
            return;
        }

        self.remove_free(this.into(), this.bucket().unwrap());

        let remaining_size = this.size() - size;

        let mut remaining_ptr = unsafe {
            let ptr: NonNull<FreeBlockHeader> = this.into();
            NonNull::new_unchecked(
                ptr.cast::<u8>().as_ptr().offset(size as isize) as *mut FreeBlockHeader
            )
        };

        let remaining = unsafe {
            ptr::write(remaining_ptr.as_ptr(), FreeBlockHeader::default());
            remaining_ptr.as_mut()
        };

        remaining.set_size(remaining_size);
        remaining.set_free(this.is_free());
        remaining.set_last(this.is_last());

        this.set_last(false);
        this.set_size(size);

        remaining.update_prev_of_next_phys();
        this.update_prev_of_next_phys();

        self.insert_free(this.into(), this.bucket().unwrap());
        self.insert_free(remaining.into(), remaining.bucket().unwrap());
    }

    fn maybe_merge_with_next_phys(&mut self, mut this_ptr: NonNull<FreeBlockHeader>)
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

                let this_bucket = this.bucket().unwrap();
                let next_bucket = next.bucket().unwrap();

                self.remove_free(this.into(), this_bucket);
                self.remove_free(next.into(), next_bucket);

                let new_size = this.size() + next.size();
                this.set_size(new_size);
                this.set_last(next.is_last());

                self.insert_free(this.into(), this.bucket().unwrap());

                this.update_prev_of_next_phys();
            }
        }
    }

    fn maybe_merge_with_neighbors_phys(&mut self, mut this_ptr: NonNull<FreeBlockHeader>)
    {
        self.maybe_merge_with_next_phys(this_ptr);

        if let Some(mut prev_ptr) = unsafe { this_ptr.as_mut() }.prev_phys()
        {
            let prev = unsafe { prev_ptr.as_mut() };

            if prev.is_free()
            {
                let prev_ptr = prev_ptr.cast::<FreeBlockHeader>();
                self.maybe_merge_with_next_phys(prev_ptr);
            }
        }
    }
}

impl<A: Allocator + Clone> Allocator for Tlsf<A>
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<NonNull<c_void>>
    {
        self.alloc_inner(layout.size)
            .map(|ptr| ptr.cast::<c_void>())
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        if let Some(ptr) = NonNull::new(ptr)
        {
            self.dealloc_inner(ptr.cast::<u8>());
        }
    }
}

impl<A: Allocator> Drop for Tlsf<A>
{
    fn drop(&mut self)
    {
        for ptr in self.allocated.iter()
        {
            unsafe {
                self.alloc.dealloc_aligned(ptr.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use core::cell::RefCell;
    use fnd::Unq;

    #[test]
    fn headers_layout()
    {
        assert_eq!(size_of::<BlockHeader>(), size_of::<usize>() * 2);
        assert_eq!(align_of::<BlockHeader>(), size_of::<usize>());
        assert_eq!(size_of::<FreeBlockHeader>(), size_of::<usize>() * 4);
        assert_eq!(align_of::<FreeBlockHeader>(), size_of::<usize>());
    }

    #[test]
    fn allocation()
    {
        let tlsf = RefCell::new(Tlsf::new());
        let mut a = Array::new_with(&tlsf);

        for i in 0..768
        {
            let _ = Unq::new_with(45, &tlsf);
            a.push(i);
        }

        for i in 0..768
        {
            assert_eq!(i, a[i]);
        }
    }
}
