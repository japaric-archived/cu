//! Exceptions

use asm;

extern "C" {
    fn __STACK_START();
    /// Reset
    pub fn __reset();
    /// Non-maskable interrupt
    pub fn __nmi();
    /// Hard fault
    pub fn __hard_fault();
}

/// Default exception handler
#[no_mangle]
pub unsafe extern "C" fn __default_handler() {
    asm::bkpt();
}

/// Cortex-M processor exceptions
#[link_section = ".exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Option<unsafe extern "C" fn()>; 16] = [Some(__STACK_START),
                                                                 Some(__reset),
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None,
                                                                 None];
