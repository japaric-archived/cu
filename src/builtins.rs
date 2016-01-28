#[no_mangle]
pub unsafe extern "C" fn __aeabi_memcpy4(dest: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
}
