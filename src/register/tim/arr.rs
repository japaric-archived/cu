//! Auto-reload register

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl From<u16> for Register {
    fn from(x: u16) -> Register {
        Register(u32::from(x))
    }
}
