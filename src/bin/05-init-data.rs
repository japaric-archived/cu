#![no_std]

extern crate cu;

#[no_mangle]
pub unsafe extern "C" fn start() {
    cu::rt::init_data();

    loop {}
}

// NOTE dummy
fn main() {}
