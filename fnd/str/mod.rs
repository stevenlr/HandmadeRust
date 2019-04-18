mod cstr;

pub use cstr::*;

pub unsafe fn strlen(mut ptr: *const u8) -> usize
{
    let mut size = 0;

    while ptr.read() != 0
    {
        size += 1;
        ptr = ptr.offset(1);
    }

    return size;
}
