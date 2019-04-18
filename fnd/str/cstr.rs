use crate::str::strlen;

pub struct CStr([u8]);

impl CStr
{
    pub unsafe fn from_bytes_null_terminated_unchecked<'a>(start: *const u8) -> &'a CStr
    {
        let size = strlen(start);
        let slice = core::slice::from_raw_parts(start, size + 1);
        return &*(slice as *const [u8] as *const CStr);
    }

    #[inline]
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error>
    {
        core::str::from_utf8(&self.0[..self.0.len() - 1])
    }
}
