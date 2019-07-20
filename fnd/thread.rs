use crate::Unq;

use core::{
    ffi::c_void,
    mem::transmute,
    ptr::{null_mut, NonNull},
};

use win32::{
    kernel32::{CloseHandle, CreateThread, WaitForSingleObject},
    DWORD, HANDLE, INFINITE, LPVOID, STACK_SIZE_PARAM_IS_A_RESERVATION,
};

pub struct Thread
{
    handle: HANDLE,
}

const STACK_SIZE: usize = 2 * 1024 * 1024;

unsafe extern "system" fn start_thread(lp_parameter: LPVOID) -> DWORD
{
    let ptr: Unq<*mut (dyn FnMut() + Send + 'static)> =
        Unq::from_raw(NonNull::new(transmute(lp_parameter)).unwrap());

    let mut closure = Unq::from_raw(NonNull::new(*ptr).unwrap());
    (&mut *closure)();

    return 0;
}

impl Thread
{
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let mut f = Some(f);
        let closure = Unq::new(move || {
            f.take()
                .expect("Re-taking a thread closure. This shouldn't happen...")();
        });

        let ptr = Unq::new(Unq::<dyn FnMut() + Send + 'static>::into_raw(closure));

        let handle = unsafe {
            CreateThread(
                null_mut(),
                STACK_SIZE,
                start_thread,
                Unq::into_raw(ptr) as *mut c_void,
                STACK_SIZE_PARAM_IS_A_RESERVATION,
                null_mut(),
            )
        };

        return Self { handle };
    }

    pub fn join(self)
    {
        unsafe {
            WaitForSingleObject(self.handle, INFINITE);
        }
    }
}

impl Drop for Thread
{
    fn drop(&mut self)
    {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
