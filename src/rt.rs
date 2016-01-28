//! Runtime

use core::{intrinsics, mem};

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

    let n = (&__DATA_END as *const _ as usize - &__DATA_START as *const _ as usize) /
            mem::size_of::<u32>();

    intrinsics::copy_nonoverlapping(&__DATA_LOAD, &mut __DATA_START, n);
}
