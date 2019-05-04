use core::ffi::c_void;

// @Todo These may not be correct on all architectures

pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;
pub type HANDLE = PVOID;
pub type HMODULE = HANDLE;
pub type HINSTANCE = HANDLE;
pub type LPCSTR = *const u8;
pub type FARPROC = *const c_void;
pub type BOOL = i32;
pub type DWORD = u32;
pub type SIZE_T = usize;
