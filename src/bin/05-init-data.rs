#![no_main]
#![no_std]

extern crate cu;

#[no_mangle]
pub extern "C" fn start() -> ! {
    unsafe {
        cu::rt::init_data();
    }

    loop {}
}
