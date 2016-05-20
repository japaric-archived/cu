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
    let ref mut rcc = register::RCC;
    let ref mut gpioc = register::GPIOC;

    // Enable Port A and Port C
    rcc.apb2enr.write(*register::rcc::apb2enr::DEFAULT.iopaen(true).iopcen(true));

    // Configure PC8 as digital output
    gpioc.crh.write(*register::gpio::crh::DEFAULT.cnf(8, 0b00).mode(8, 0b10));
}

fn loop_() {
    let ref pa0 = bb::GPIOA.idr[0];
    let ref pc8 = bb::GPIOC.odr[8];

    loop {
        if pa0.is_set() {
            if pc8.is_set() {
                pc8.clear();
            } else {
                pc8.set();
            }
        }
    }
}

// NOTE dummy
fn main() {}
