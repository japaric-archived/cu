//! Port configuration register high

use rt;

/// Reset value
pub const DEFAULT: Register = Register(0x4444_4444);

/// Register
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Sets `pin` configuration
    pub fn cnf(&mut self, pin: u32, state: u32) -> &mut Self {
        match (state, pin) {
            (0b00...0b11, 8...15) => {
                const MASK: u32 = 0b11;

                let offset = 4 * (pin - 8) + 2;

                self.0 &= !(MASK << offset);
                self.0 |= state << offset;
            }
            _ => rt::abort(),
        }

        self
    }

    /// Sets `pin` mode
    pub fn mode(&mut self, pin: u32, state: u32) -> &mut Self {
        match (state, pin) {
            (0b00...0b11, 8...15) => {
                const MASK: u32 = 0b11;

                let offset = 4 * (pin - 8);

                self.0 &= !(MASK << offset);
                self.0 |= state << offset;
            }
            _ => rt::abort(),
        }

        self
    }
}
