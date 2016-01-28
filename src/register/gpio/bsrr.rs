//! Port bit set/reset register

use rt;

/// Reset value
pub const DEFAULT: Register = Register(0x0000_0000);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Resets `pin`
    pub fn reset(&mut self, pin: u32) -> &mut Self {
        match pin {
            0...15 => {
                let offset = pin + 16;

                self.0 |= 1 << offset;
            }
            _ => rt::abort(),
        }

        self
    }

    /// Sets `pin`
    pub fn set(&mut self, pin: u32) -> &mut Self {
        match pin {
            0...15 => {
                let offset = pin;

                self.0 |= 1 << offset;
            }
            _ => rt::abort(),
        }

        self
    }
}
