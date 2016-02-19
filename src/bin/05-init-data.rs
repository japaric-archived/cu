#![no_std]

extern crate cu;

#[no_mangle]
pub unsafe extern "C" fn __reset() {
    cu::rt::init_data();

    loop {}
}

// NOTE dummy
fn main() {}
