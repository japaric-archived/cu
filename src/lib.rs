//! STM32VLDISCOVERY

#![feature(asm)]
#![feature(lang_items)]

#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

#![no_std]

extern crate rustc_builtins;

pub mod asm;
pub mod bb;
pub mod exception;
pub mod interrupt;
pub mod register;
pub mod rt;

mod lang_items;
