//! The aarch64_bootloader crate
//!
//! This crate is an implementation of an AArch64 bare-metal kernel for learning purposes.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
