//! General Purpose I/O

pub mod bsrr;
pub mod crh;

use register::{Rw, Wo};

/// Register block
#[repr(C)]
pub struct Block {
    crl: u32,
    /// 0x04
    pub crh: Rw<crh::Register>,
    idr: u32,
    odr: u32,
    /// 0x10
    pub bsrr: Wo<bsrr::Register>,
    brr: u32,
    lckr: u32,
}
