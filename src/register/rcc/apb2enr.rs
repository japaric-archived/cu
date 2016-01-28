//! APB2 peripheral clock enable register

/// Reset value
pub const DEFAULT: Register = Register(0);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    bits! {
        iopaen: 2,
        iopcen: 4,
    }
}
