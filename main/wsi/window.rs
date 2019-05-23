use super::{Event, EventQueue};
use core::{
    pin::Pin,
    ptr::{null, null_mut},
};
use fnd::{containers::Array, Unq};
use vk::{builders::*, types::*};
use win32::{self, kernel32, user32};

fn queue_event(window: win32::HWND, event: Event)
{
    let queue =
        unsafe { user32::GetWindowLongPtrA(window, win32::GWLP_USERDATA) as *const EventQueue };

    if !queue.is_null()
    {
        let queue: &EventQueue = unsafe { &*queue };
        queue.queue_event(event);
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: win32::HWND,
    msg: win32::UINT,
    w_param: win32::WPARAM,
    l_param: win32::LPARAM,
) -> win32::LRESULT
{
    match msg
    {
        win32::WM_DESTROY =>
        {
            queue_event(hwnd, Event::DestroyWindow);
            0
        }
        _ => user32::DefWindowProcA(hwnd, msg, w_param, l_param),
    }
}

pub struct Window
{
    hinstance: win32::HINSTANCE,
    window: win32::HWND,
    queue: Pin<Unq<EventQueue>>,
}

impl Window
{
    pub fn new(width: i32, height: i32, title: &str) -> Option<Self>
    {
        let hinstance = unsafe { kernel32::GetModuleHandleA(0 as _) as win32::HINSTANCE };

        let wnd_class_name = b"HandmadeRustClass\0".as_ptr();

        let wnd_class = win32::WNDCLASSA {
            style: win32::CS_VREDRAW | win32::CS_HREDRAW,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: wnd_class_name,
        };

        // @Todo Make CString
        let mut title_z: Array<_> = title.bytes().collect();
        title_z.push(0);

        let window = unsafe {
            user32::RegisterClassA(&wnd_class);
            user32::CreateWindowExA(
                0,
                wnd_class_name,
                title_z.as_ptr(),
                win32::WS_OVERLAPPEDWINDOW,
                win32::CW_USEDEFAULT,
                win32::CW_USEDEFAULT,
                width,
                height,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut(),
            )
        };

        if window == null_mut()
        {
            None
        }
        else
        {
            let event_queue = Unq::pin(EventQueue::new());

            unsafe {
                user32::ShowWindow(window, win32::SW_SHOW);
                user32::SetWindowLongPtrA(
                    window,
                    win32::GWLP_USERDATA,
                    &*event_queue as *const _ as _,
                );
            }

            Some(Self {
                hinstance,
                window,
                queue: event_queue,
            })
        }
    }

    pub fn create_vk_surface(&self, vk_instance: &vk::Instance) -> Result<VkSurfaceKHR, VkResult>
    {
        let create_info = VkWin32SurfaceCreateInfoKHRBuilder::new()
            .hinstance(self.hinstance)
            .hwnd(self.window);

        vk_instance
            .create_win_32_surface_khr(&create_info, None)
            .map(|p| p.1)
    }

    fn handle_events(&self)
    {
        unsafe {
            let mut msg: win32::MSG = core::mem::zeroed();
            while user32::PeekMessageA(&mut msg, self.window, 0, 0, win32::PM_REMOVE) > 0
            {
                user32::TranslateMessage(&msg);
                user32::DispatchMessageA(&msg);
            }
        }
    }

    #[inline]
    pub fn events_loop(&self, mut f: impl FnMut(&Event) -> bool)
    {
        'outer_loop: loop
        {
            self.handle_events();

            while let Some(event) = self.queue.poll_event()
            {
                if !f(&event)
                {
                    break 'outer_loop;
                }
            }
        }
    }
}

impl Drop for Window
{
    fn drop(&mut self)
    {
        unsafe {
            user32::SetWindowLongPtrA(self.window, win32::GWLP_USERDATA, 0);
            user32::DestroyWindow(self.window);
        }
    }
}
