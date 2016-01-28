//! Basic timers

use bb::Rw;

/// Register block
#[repr(C)]
pub struct Block {
    cr1: [u32; 32],
    cr2: [u32; 32],
    _0: [u32; 32],
    dier: [u32; 32],
    /// 0x10
    pub sr: Sr,
    egr: [u32; 32],
    _1: [u32; 3 * 32],
    cnt: [u32; 32],
    psc: [u32; 32],
    arr: [u32; 32],
}

/// Status register
#[repr(C)]
pub struct Sr {
    /// Update interrupt flag
    pub uif: Rw,
    _0: [u32; 31],
}
