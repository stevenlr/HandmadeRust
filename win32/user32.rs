use crate::types::*;

#[link(name = "user32")]
extern "system" {
    pub fn RegisterClassA(lpWndClass: *const WNDCLASSA) -> ATOM;
    pub fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> BOOL;
    pub fn PeekMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
    pub fn DispatchMessageA(lpMsg: *const MSG) -> LRESULT;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn GetWindowLongPtrA(hWnd: HWND, nIndex: i32) -> LONG_PTR;
    pub fn SetWindowLongPtrA(hWnd: HWND, nIndex: i32, dwNewLong: LONG_PTR) -> LONG_PTR;
}
