use crate::types::*;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetModuleHandleA(module_name: LPCSTR) -> HMODULE;
    pub fn GetProcAddress(module: HMODULE, fn_name: LPCSTR) -> FARPROC;
    pub fn GetProcessHeap() -> HANDLE;
    pub fn HeapAlloc(heap: HANDLE, flags: DWORD, byte: SIZE_T) -> LPVOID;
    pub fn HeapFree(heap: HANDLE, flags: DWORD, mem: LPVOID) -> BOOL;
    pub fn LoadLibraryA(lib_name: LPCSTR) -> HMODULE;
    pub fn CreateThread(
        lpThreadAttributes: *mut SECURITY_ATTRIBUTES,
        dwStackSize: SIZE_T,
        lpStartAddress: ThreadProc,
        lpParameter: LPVOID,
        dwCreationFlags: DWORD,
        lpThreadId: *mut DWORD,
    ) -> HANDLE;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    pub fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
}
