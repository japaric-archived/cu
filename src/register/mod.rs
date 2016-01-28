//! Registers

use core::intrinsics;

macro_rules! bits {
    ($($name:ident: $offset:expr),+,) => {
        $(
            /// Set/clear bit
            pub fn $name(&mut self, on: bool) -> &mut Self {
                if on {
                    self.0 |= 1 << $offset;
                } else {
                    self.0 &= !(1 << 4);
                }

                self
            }
        )+
    }
}

pub mod gpio;
pub mod itm;
pub mod nvic;
pub mod rcc;
pub mod tim;

extern "C" {
    #[link_name = "__GPIOA"]    pub static mut GPIOA: gpio::Block;
    #[link_name = "__GPIOC"]    pub static mut GPIOC: gpio::Block;
    #[link_name = "__NVIC"]    pub static mut NVIC: nvic::Block;
    #[link_name = "__RCC"]    pub static mut RCC: rcc::Block;
    #[link_name = "__TIM7"]    pub static mut TIM7: tim::Block;
}

/// Read/write register
#[repr(C)]
pub struct Rw<T>(T) where T: Copy;

impl<T> Rw<T>
    where T: Copy
{
    /// Reads the register value
    pub fn read(&self) -> T {
        unsafe { intrinsics::volatile_load(&self.0) }
    }

    /// Updates the register value
    pub fn update<F>(&mut self, f: F)
        where F: FnOnce(&mut T) -> &mut T
    {
        let mut value = self.read();
        f(&mut value);
        self.write(value);
    }

    /// Writes `value` to the register
    pub fn write<U>(&mut self, value: U)
        where T: From<U>
    {
        unsafe {
            intrinsics::volatile_store(&mut self.0, T::from(value));
        }
    }
}

/// Write-only register
#[repr(C)]
pub struct Wo<T>(T) where T: Copy;

impl<T> Wo<T>
    where T: Copy
{
    /// Writes `value` to the register
    pub fn write<U>(&mut self, value: U)
        where T: From<U>
    {
        unsafe {
            intrinsics::volatile_store(&mut self.0, T::from(value));
        }
    }
}
