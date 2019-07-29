use fnd::*;

#[cfg(target_env = "msvc")]
#[link(name = "msvcrt")]
extern "C" {
    fn exit(status: i32) -> !;
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !
{
    println!("{}", info);
    unsafe {
        exit(1);
    }
}

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
