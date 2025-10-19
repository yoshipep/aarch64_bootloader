//! The aarch64_bootloader crate
//!
//! This crate is an implementation of an AArch64 bootloader to load the [aarch64_kernel](https://github.com/yoshipep/aarch64_kernel) for learning purposes.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod exception;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
