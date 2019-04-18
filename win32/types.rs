use core::ffi::c_void;

pub type HMODULE = usize;
pub type LPCSTR = *const u8;
pub type FARPROC = *const c_void;