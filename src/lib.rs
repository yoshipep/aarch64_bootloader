//! AArch64 Bootloader
//!
//! This crate implements a minimal bootloader for the AArch64 architecture,
//! designed for educational purposes

#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod parsers;
pub mod exception;
pub mod drivers;
pub mod utilities;

/// Panic handler for the bootloader
///
/// When a panic occurs, this handler is invoked. Currently, it enters an
/// infinite loop, halting execution. In a real bootloader, this might
/// perform cleanup or print diagnostic information.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
