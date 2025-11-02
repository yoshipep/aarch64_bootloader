//! Hexadecimal printing utilities
//!
//! This module provides functions to format and print integer values in
//! hexadecimal format. These are particularly useful for debugging and
//! exception handling where standard formatting traits are not available
//! in a `no_std` environment.
//!
//! All functions output directly to the UART using the PL011 driver.

use crate::drivers::uart::pl011;

/// Lookup table for hexadecimal digit conversion
const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// Prints a u64 value as a 16-digit hexadecimal number to UART
///
/// This function formats the value as a zero-padded 16-character hexadecimal
/// string (e.g., `0000000000000042` for the value 66).
///
pub fn print_hex_u64(mut value: u64) {
    let mut buf = [0u8; 16];

    // Convert to hex digits (right to left)
    for i in (0..16).rev() {
        buf[i] = HEX_CHARS[(value & 0xF) as usize];
        value >>= 4;
    }

    pl011::print(&buf);
}

/// Prints an 8-bit value as a 2-digit hexadecimal number to UART
///
/// This function formats the value as a zero-padded 2-character hexadecimal
/// string using uppercase letters (e.g., `2A` for the value 42).
pub fn print_hex_u8(val: u8) {
    let mut buf = [0u8; 2];

    for i in 0..2 {
        let nibble = ((val >> ((1 - i) * 4)) & 0xF) as u8;
        if nibble < 10 {
            buf[i] = b'0' + nibble;
        } else {
            buf[i] = b'A' + (nibble - 10);
        };
    }

    pl011::print(&buf);
}
