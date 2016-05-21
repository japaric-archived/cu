//! Bit-banded peripherals

use core::ptr;

pub mod gpio;
pub mod rcc;
pub mod tim;

extern "C" {
    #[link_name = "__BB_GPIOA"]    pub static GPIOA: gpio::Block;
    #[link_name = "__BB_GPIOC"]    pub static GPIOC: gpio::Block;
    #[link_name = "__BB_RCC"]    pub static RCC: rcc::Block;
    #[link_name = "__BB_TIM7"]    pub static TIM7: tim::Block;
}

/// Bit-banded bit with read-only access
#[repr(C)]
pub struct Ro(u32);

impl Ro {
    /// Checks if the bit is set
    pub fn is_set(&self) -> bool {
        unsafe { ptr::read_volatile(&self.0) != 0 }
    }
}

/// Bit-banded bit with read/write access
#[repr(C)]
pub struct Rw(u32);

impl Rw {
    /// Clears the bit
    pub fn clear(&self) {
        unsafe {
            ptr::write_volatile(&self.0 as *const _ as *mut _, 0);
        }
    }

    /// Checks if the bit is set
    pub fn is_set(&self) -> bool {
        unsafe { ptr::read_volatile(&self.0) != 0 }
    }

    /// Sets the bit
    pub fn set(&self) {
        unsafe {
            ptr::write_volatile(&self.0 as *const _ as *mut _, 1);
        }
    }
}

// TODO bit-band routine for RAM?
