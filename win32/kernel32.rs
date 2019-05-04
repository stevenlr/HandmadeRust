use crate::types::*;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleA(module_name: LPCSTR) -> HMODULE;
    pub fn GetProcAddress(module: HMODULE, fn_name: LPCSTR) -> FARPROC;
    pub fn GetProcessHeap() -> HANDLE;
    pub fn HeapAlloc(heap: HANDLE, flags: DWORD, byte: SIZE_T) -> LPVOID;
    pub fn HeapFree(heap: HANDLE, flags: DWORD, mem: LPVOID) -> BOOL;
    pub fn LoadLibraryA(lib_name: LPCSTR) -> HMODULE;
}
