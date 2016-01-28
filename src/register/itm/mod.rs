//! Instrumentation Trace Macrocell

use register::{Rw, Wo};

pub mod port;
pub mod ter;
pub mod tcr;

/// Register block
#[repr(C)]
pub struct Block {
    /// 0x0000
    pub port: [port::Register; 32],
    _0: [u32; 864],
    /// 0x0E00
    pub ter: Rw<ter::Register>,
    _1: [u32; 15],
    /// 0x0E40
    tpr: Rw<u32>,
    _2: [u32; 15],
    /// 0x0E80
    pub tcr: Rw<tcr::Register>,
    _3: [u32; 75],
    /// 0x0FB0
    lar: Wo<u32>,
}
