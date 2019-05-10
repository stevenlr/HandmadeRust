use core::{cmp, hash::Hasher, mem::transmute, ptr::copy_nonoverlapping};

const C: i32 = 2;
const D: i32 = 4;

pub struct SipHash
{
    v0: u64,
    v1: u64,
    v2: u64,
    v3: u64,
}

#[inline]
fn round(mut v0: u64, mut v1: u64, mut v2: u64, mut v3: u64) -> (u64, u64, u64, u64)
{
    v0 = v0.wrapping_add(v1);
    v2 = v2.wrapping_add(v3);
    v1 = v1.rotate_left(13);
    v3 = v3.rotate_left(16);
    v1 ^= v0;
    v3 ^= v2;
    v0 = v0.rotate_left(32);
    v2 = v2.wrapping_add(v1);
    v0 = v0.wrapping_add(v3);
    v1 = v1.rotate_left(17);
    v3 = v3.rotate_left(21);
    v1 ^= v2;
    v3 ^= v0;
    v2 = v2.rotate_left(32);
    return (v0, v1, v2, v3);
}

impl Default for SipHash
{
    fn default() -> Self
    {
        Self::new(0, 0)
    }
}

impl SipHash
{
    pub fn new(k0: u64, k1: u64) -> Self
    {
        Self {
            v0: k0 ^ 0x736f6d6570736575u64,
            v1: k1 ^ 0x646f72616e646f6du64,
            v2: k0 ^ 0x6c7967656e657261u64,
            v3: k1 ^ 0x7465646279746573u64,
        }
    }

    fn bytes_to_m(&self, bytes: &[u8], start: usize) -> u64
    {
        let mut value: u64 = 0;
        let size = cmp::min(bytes.len() - start, 8);
        unsafe {
            copy_nonoverlapping(
                bytes.as_ptr().offset(start as isize),
                transmute(&mut value),
                size,
            );
        }
        if size < 8
        {
            value |= (bytes.len() as u64 & 0xff) << 56;
        }
        return value;
    }

    fn round(&mut self)
    {
        let v = round(self.v0, self.v1, self.v2, self.v3);
        self.v0 = v.0;
        self.v1 = v.1;
        self.v2 = v.2;
        self.v3 = v.3;
    }
}

impl Hasher for SipHash
{
    fn finish(&self) -> u64
    {
        let mut v0 = self.v0;
        let mut v1 = self.v1;
        let mut v2 = self.v2 ^ 0xff;
        let mut v3 = self.v3;

        for _ in 0..D
        {
            let v = round(v0, v1, v2, v3);
            v0 = v.0;
            v1 = v.1;
            v2 = v.2;
            v3 = v.3;
        }

        return v0 ^ v1 ^ v2 ^ v3;
    }

    fn write(&mut self, bytes: &[u8])
    {
        for start in (0..bytes.len() + 1).step_by(8)
        {
            let m = self.bytes_to_m(bytes, start);
            self.v3 ^= m;

            for _ in 0..C
            {
                self.round();
            }

            self.v0 ^= m;
        }
    }
}
