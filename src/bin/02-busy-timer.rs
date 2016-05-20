#![no_std]

extern crate cu;

use cu::{bb, register};

#[no_mangle]
pub extern "C" fn start() {
    unsafe {
        setup();
    }

    loop_();
}

unsafe fn setup() {
    let ref mut gpioc = register::GPIOC;
    let ref mut rcc = register::RCC;
    let ref mut tim7 = register::TIM7;

    // Enable TIM7 and GPIOC
    rcc.apb1enr.write(*register::rcc::apb1enr::DEFAULT.tim7en(true));
    rcc.apb2enr.write(*register::rcc::apb2enr::DEFAULT.iopcen(true));

    // Set PC8 as digital output
    gpioc.crh.write(*register::gpio::crh::DEFAULT.cnf(8, 0b00).mode(8, 0b10));

    // Configure TIM7 for 1 Hz timeouts
    tim7.psc.write(999);
    tim7.arr.write(8_000);
    tim7.cr1.write(*register::tim::cr1::DEFAULT.cen(true));
}

fn loop_() {
    let ref uif = bb::TIM7.sr.uif;
    let ref pc8 = bb::GPIOC.odr[8];

    loop {
        if uif.is_set() {
            uif.clear();

            if pc8.is_set() {
                pc8.clear()
            } else {
                pc8.set()
            }
        }
    }
}

// NOTE dummy
fn main() {}
