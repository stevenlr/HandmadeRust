use crate::types::*;

#[link(name = "kernel32")]
extern "system"
{
    pub fn LoadLibraryA(lib_name: LPCSTR) -> HMODULE;
    pub fn GetProcAddress(module: HMODULE, fn_name: LPCSTR) -> FARPROC;
}