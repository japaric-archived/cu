//! Interrupt Set-enable 2 register

/// Reset value
pub const DEFAULT: Register = Register(0x0000_0000);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Interrupt set-enable bits
    pub fn setena(&mut self, i: u32) -> &mut Self {
        match i {
            32...63 => {
                self.0 |= 1 << (i - 32);
            }
            _ => ::rt::abort(),
        }

        self
    }
}
