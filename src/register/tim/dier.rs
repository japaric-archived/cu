//! DMA/interrupt enable register

/// Reset value
pub const DEFAULT: Register = Register(0x0000_0000);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    bits! {
        uie: 0,
    }
}
