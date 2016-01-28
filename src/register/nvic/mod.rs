//! Nested vector interrupt controller

pub mod iser1;

use register::Rw;

/// Register block
#[repr(C)]
pub struct Block {
    iser0: u32,
    /// 0x04
    pub iser1: Rw<iser1::Register>,
    iser2: u32,
}
