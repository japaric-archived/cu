#![no_std]

extern crate cu;

#[no_mangle]
pub unsafe extern "C" fn __reset() {
    loop {}
}

// NOTE dummy
fn main() {}
