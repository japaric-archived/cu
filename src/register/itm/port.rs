//! Stimulus port

use core::ptr;

/// Register
#[repr(C)]
pub struct Register(u32);

impl Register {
    /// Checks if the stimulus port FIFO is full
    pub fn is_full(&self) -> bool {
        unsafe { ptr::read_volatile(&self.0) == 0 }
    }

    /// Writes a character to the stimulus port FIFO
    pub fn write(&mut self, c: u8) {
        unsafe {
            ptr::write_volatile(&mut self.0 as *mut _ as *mut u8, c);
        }
    }
}
