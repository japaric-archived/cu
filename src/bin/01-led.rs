#![no_std]

extern crate cu;

use cu::register;

#[no_mangle]
pub extern "C" fn start() -> ! {
    unsafe {
        let ref mut rcc = register::RCC;
        let ref mut gpioc = register::GPIOC;

        // Enable Port C
        rcc.apb2enr.write(*register::rcc::apb2enr::DEFAULT.iopcen(true));

        // Set PC8 as digital output
        gpioc.crh.write(*register::gpio::crh::DEFAULT.cnf(8, 0b00).mode(8, 0b10));

        // Set PC8 high
        gpioc.bsrr.write(*register::gpio::bsrr::DEFAULT.set(8));

        loop {}
    }
}

// NOTE dummy
fn main() {}
