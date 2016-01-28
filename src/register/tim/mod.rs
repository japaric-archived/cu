//! Basic timers

pub mod arr;
pub mod dier;
pub mod cr1;
pub mod egr;
pub mod psc;

use register::{Rw, Wo};

/// Register block
#[repr(C)]
pub struct Block {
    /// 0x00
    pub cr1: Rw<cr1::Register>,
    cr2: u32,
    _0: u32,
    /// 0x0C
    pub dier: Rw<dier::Register>,
    sr: u32,
    /// 0x14
    pub egr: Wo<egr::Register>,
    _1: [u32; 3],
    cnt: u32,
    /// 0x28
    pub psc: Rw<psc::Register>,
    /// 0x2C
    pub arr: Rw<arr::Register>,
}
