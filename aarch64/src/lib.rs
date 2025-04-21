#![cfg_attr(not(test), no_std)]

pub mod gicv3;
pub mod mmu;
pub mod pl011;
pub mod regs;
pub mod semihosting;

mod tests;
