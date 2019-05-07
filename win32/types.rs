use core::ffi::c_void;

// @Todo These may not be correct on all architectures

pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;
pub type HANDLE = PVOID;
pub type HMODULE = HANDLE;
pub type HICON = HANDLE;
pub type HCURSOR = HANDLE;
pub type HBRUSH = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HWND = HANDLE;
pub type HMENU = HANDLE;
pub type LPCSTR = *const u8;
pub type FARPROC = *const c_void;
pub type BOOL = i32;
pub type DWORD = u32;
pub type SIZE_T = usize;
pub type UINT = u32;
pub type LPARAM = isize;
pub type WPARAM = usize;
pub type LRESULT = isize;
pub type WORD = u16;
pub type ATOM = WORD;
pub type LONG = i32;
pub type LPMSG = *mut MSG;

pub type WNDPROC =
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

pub const CS_BYTEALIGNCLIENT: UINT = 0x1000;
pub const CS_BYTEALIGNWINDOW: UINT = 0x2000;
pub const CS_CLASSDC: UINT = 0x0040;
pub const CS_DBLCLKS: UINT = 0x0008;
pub const CS_DROPSHADOW: UINT = 0x00020000;
pub const CS_GLOBALCLASS: UINT = 0x4000;
pub const CS_HREDRAW: UINT = 0x0002;
pub const CS_NOCLOSE: UINT = 0x0200;
pub const CS_OWNDC: UINT = 0x0020;
pub const CS_PARENTDC: UINT = 0x0080;
pub const CS_SAVEBITS: UINT = 0x0800;
pub const CS_VREDRAW: UINT = 0x0001;

pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
pub const WS_EX_LTRREADING: DWORD = 0x00000000;
pub const WS_EX_MDICHILD: DWORD = 0x00000040;
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = (WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE);
pub const WS_EX_PALETTEWINDOW: DWORD = (WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST);
pub const WS_EX_RIGHT: DWORD = 0x00001000;
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
pub const WS_EX_RTLREADING: DWORD = 0x00002000;
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_EX_TOPMOST: DWORD = 0x00000008;
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;

pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_POPUP: DWORD = 0x80000000;
pub const WS_CHILD: DWORD = 0x40000000;
pub const WS_MINIMIZE: DWORD = 0x20000000;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_DISABLED: DWORD = 0x08000000;
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;
pub const WS_MAXIMIZE: DWORD = 0x01000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_BORDER: DWORD = 0x00800000;
pub const WS_DLGFRAME: DWORD = 0x00400000;
pub const WS_VSCROLL: DWORD = 0x00200000;
pub const WS_HSCROLL: DWORD = 0x00100000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_GROUP: DWORD = 0x00020000;
pub const WS_TABSTOP: DWORD = 0x00010000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_TILED: DWORD = WS_OVERLAPPED;
pub const WS_ICONIC: DWORD = WS_MINIMIZE;
pub const WS_SIZEBOX: DWORD = WS_THICKFRAME;
pub const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW: DWORD =
    (WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX);
pub const WS_POPUPWINDOW: DWORD = (WS_POPUP | WS_BORDER | WS_SYSMENU);
pub const WS_CHILDWINDOW: DWORD = (WS_CHILD);

pub const CW_USEDEFAULT: i32 = -2147483648;

pub const SW_HIDE: i32 = 0;
pub const SW_SHOWNORMAL: i32 = 1;
pub const SW_NORMAL: i32 = 1;
pub const SW_SHOWMINIMIZED: i32 = 2;
pub const SW_SHOWMAXIMIZED: i32 = 3;
pub const SW_MAXIMIZE: i32 = 3;
pub const SW_SHOWNOACTIVATE: i32 = 4;
pub const SW_SHOW: i32 = 5;
pub const SW_MINIMIZE: i32 = 6;
pub const SW_SHOWMINNOACTIVE: i32 = 7;
pub const SW_SHOWNA: i32 = 8;
pub const SW_RESTORE: i32 = 9;
pub const SW_SHOWDEFAULT: i32 = 10;
pub const SW_FORCEMINIMIZE: i32 = 11;

pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;
pub const PM_NOYIELD: UINT = 0x0002;

#[repr(C)]
pub struct WNDCLASSA
{
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCSTR,
    pub lpszClassName: LPCSTR,
}

#[repr(C)]
pub struct POINT
{
    x: LONG,
    y: LONG,
}

#[repr(C)]
pub struct MSG
{
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}
