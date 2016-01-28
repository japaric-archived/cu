//! Trace Control Register

/// Register

/// Reset value
pub const DEFAULT: Register = Register(0x0000_0000);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    bits! {
        itmena: 0,
        tsena: 1,
        syncena: 2,
        dwtena: 3,
        swoena: 4,
    }
}
