//! General Purpose Input/Output

use bb::{Ro, Rw};

/// Register block
#[repr(C)]
pub struct Block {
    crl: [u32; 32],
    crh: [u32; 32],
    /// 0x08
    pub idr: [Ro; 32],
    /// 0x0C
    pub odr: [Rw; 32],
    bsrr: [u32; 32],
    brr: [u32; 32],
    lckr: [u32; 32],
}
