//! STM32VLDISCOVERY

#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(lang_items)]

#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

#![no_std]

// TODO remove this in favor of compiler-rt
#[doc(hidden)]
pub mod builtins;

pub mod asm;
pub mod bb;
pub mod exception;
pub mod interrupt;
pub mod register;
pub mod rt;

mod lang_items;
