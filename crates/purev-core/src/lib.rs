//! Core `#![no_std]` + `alloc` RV64IMAC emulator engine.

#![no_std]

extern crate alloc;

pub mod bus;
pub mod cpu;
pub mod elf;
pub mod journal;
pub mod machine;

pub use machine::Machine;
pub use purev_isa::{Addr, IsaConfig, RegValue, Register, Trap};
