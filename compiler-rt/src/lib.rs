//! Compiler-rt "builtins" implemented in Rust

#![deny(warnings)]

#![no_builtins]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub unsafe extern "C" fn __aeabi_memcpy4(dest: *mut u8, src: *const u8, n: usize) {
    rlibc::memcpy(dest, src, n);
}
