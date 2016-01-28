//! Trace Enable Register

use rt;

/// Reset value
pub const DEFAULT: Register = Register(0x0000_0000);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Enables a stimulus `port`
    pub fn enable(&mut self, port: u32) -> &mut Self {
        match port {
            0...31 => {
                let offset = port;

                self.0 |= 1 << offset;
            }
            _ => rt::abort(),
        }

        self
    }
}
