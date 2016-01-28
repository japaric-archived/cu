//! Assembly

/// BreaKPoinT
pub fn bkpt() {
    unsafe {
        asm!("bkpt" :::: "volatile");
    }
}

/// Wait For Interrupt
pub fn wfi() {
    unsafe {
        asm!("wfi" :::: "volatile");
    }
}
