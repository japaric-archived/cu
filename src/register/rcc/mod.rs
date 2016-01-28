//! Register and Clock Control

pub mod apb2enr;
pub mod apb1enr;

use register::Rw;

/// Register block
#[repr(C)]
pub struct Block {
    cr: u32,
    cfgr: u32,
    cir: u32,
    apb2rstr: u32,
    apb1rstr: u32,
    ahbenr: u32,
    /// 0x18
    pub apb2enr: Rw<apb2enr::Register>,
    /// 0x1C
    pub apb1enr: Rw<apb1enr::Register>,
    bdcr: u32,
    csr: u32,
    cfgr2: u32,
}
