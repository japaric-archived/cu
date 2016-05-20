//! Exceptions

use asm;

extern "C" {
    /// The start of the stack
    static __STACK_START: u32;
    /// Reset
    pub fn start() -> !;
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

/// Points at the start of the stack. This is used as the initial value of the stack pointer.
#[link_section = ".stack_pointer"]
#[no_mangle]
pub static __STACK_POINTER: &'static u32 = &__STACK_START;

/// Program entry point: The reset function.
#[link_section = ".reset"]
#[no_mangle]
pub static __RESET: Option<unsafe extern "C" fn() -> !> = Some(start);

/// Cortex-M processor exceptions
#[link_section = ".exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Option<unsafe extern "C" fn()>; 14] = [None,
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
