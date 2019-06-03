pub fn murmur3_32(key: &impl AsRef<[u8]>) -> u32
{
    #[inline]
    fn fmix32(mut h: u32) -> u32
    {
        h ^= h >> 16;
        h = h.wrapping_mul(0x85ebca6b);
        h ^= h >> 13;
        h = h.wrapping_mul(0xc2b2ae35);
        h ^= h >> 16;
        return h;
    }

    let key = key.as_ref();
    let nblocks = key.len() / 4;

    let mut h1 = 0u32; // Seed

    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;

    //----------
    // body

    let blocks = unsafe { core::slice::from_raw_parts(key.as_ptr() as *const u32, nblocks) };

    for mut k1 in blocks.iter().copied()
    {
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    //----------
    // tail

    let tail = &key[nblocks * 4..];
    let mut k1 = 0u32;

    if tail.len() >= 1
    {
        if tail.len() >= 2
        {
            if tail.len() == 3
            {
                k1 ^= (tail[2] as u32) << 16;
            }

            k1 ^= (tail[1] as u32) << 8;
        }

        k1 ^= tail[0] as u32;
        k1 = k1.wrapping_mul(C1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(C2);
        h1 ^= k1;
    }

    //----------
    // finalization

    h1 ^= key.len() as u32;
    h1 = fmix32(h1);
    return h1;
}
