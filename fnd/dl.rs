use crate::containers::{String, WString};
use win32::{
    kernel32::{FreeLibrary, GetProcAddress, LoadLibraryW},
    HMODULE,
};

pub struct DynamicLibrary(HMODULE);

impl DynamicLibrary
{
    pub fn load(name: &str) -> Option<Self>
    {
        // @Todo Use small stack allocator when UTF-16 length is small.
        // Same thing in get_symbol.
        let mut name = WString::from_str(name);
        name.push('\0');

        let module = unsafe { LoadLibraryW(name.as_ptr()) };

        if module.is_null()
        {
            None
        }
        else
        {
            Some(Self(module))
        }
    }

    pub fn get_symbol<T>(&self, name: &str) -> Option<*mut T>
    {
        let mut name = String::from_str(name);
        name.push(0 as char);

        return unsafe { self.get_symbol_from_bytes_null_terminated(name.as_bytes()) };
    }

    pub unsafe fn get_symbol_from_bytes_null_terminated<T>(&self, name: &[u8]) -> Option<*mut T>
    {
        let ptr = GetProcAddress(self.0, name.as_ptr()) as *mut T;

        if ptr.is_null()
        {
            None
        }
        else
        {
            Some(ptr)
        }
    }
}

impl Drop for DynamicLibrary
{
    fn drop(&mut self)
    {
        unsafe {
            FreeLibrary(self.0);
        }
    }
}
