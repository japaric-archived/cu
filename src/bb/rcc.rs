//! Register and Clock Control

use bb::Rw;

/// Register block
#[repr(C)]
pub struct Block {
    cr: [u32; 32],
    cfgr: [u32; 32],
    cir: [u32; 32],
    apb2rstr: [u32; 32],
    apb1rstr: [u32; 32],
    ahbenr: [u32; 32],
    /// 0x18
    pub apb2enr: Apb2enr,
    apb1enr: [u32; 32],
    bdcr: [u32; 32],
    csr: [u32; 32],
    cfgr2: [u32; 32],
}

/// APB2 peripheral clock enable
#[repr(C)]
pub struct Apb2enr {
    afioen: u32,
    _0: u32,
    iopaen: u32,
    iopben: u32,
    /// I/O port C clock enable
    pub iopcen: Rw,
    iopden: u32,
    iopeen: u32,
    iopfen: u32,
    iopgen: u32,
    adc1en: u32,
    _1: u32,
    tim1en: u32,
    spi1en: u32,
    _2: u32,
    usart1en: u32,
    _3: u32,
    tim15en: u32,
    tim16e: u32,
    tim17e: u32,
    _4: [u32; 13],
}
