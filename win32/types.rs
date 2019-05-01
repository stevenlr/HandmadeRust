use core::ffi::c_void;

pub type PVOID      = *mut c_void;
pub type HANDLE     = PVOID;
pub type HMODULE    = HANDLE;
pub type HINSTANCE  = HANDLE;
pub type LPCSTR     = *const u8;
pub type FARPROC    = *const c_void;