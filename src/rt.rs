//! Runtime

use core::ptr;

/// Abort
pub fn abort() -> ! {
    ::asm::bkpt();

    loop {}
}

/// Initializes the .data section
pub unsafe fn init_data() {
    extern "C" {
        static __DATA_LOAD: u32;

        static mut __DATA_END: u32;
        static mut __DATA_START: u32;
    }

    // NOTE: wrapping_sub and bitshift are used here to avoid panics due to overflow checks and
    // divide by zero.
    let n = (&__DATA_END as *const _ as usize).wrapping_sub(&__DATA_START as *const _ as usize) >>
            2;

    ptr::copy_nonoverlapping(&__DATA_LOAD, &mut __DATA_START, n);
}
