//! Memory-mapped I/O utilities
//!
//! This module provides safe wrappers around volatile memory operations for
//! interacting with memory-mapped hardware registers. All functions use
//! volatile reads and writes to ensure the compiler doesn't optimize away
//! hardware accesses.

use core::ptr::{read_volatile, write_volatile};

/// Reads a 32-bit value from a memory-mapped I/O register
///
/// Performs a volatile read from the register at `base + offset`. The volatile
/// operation ensures the compiler won't optimize away the read.
///
/// # Safety
///
/// The caller must ensure:
/// - `base + offset` points to a valid, accessible 32-bit register
/// - The address is properly aligned for 32-bit access
/// - Reading from this register won't cause side effects that violate program invariants
pub unsafe fn read_mmio32(base: usize, offset: usize) -> u32 {
    unsafe {
        let ptr = (base as *const u8).add(offset) as *const u32;
        return read_volatile(ptr);
    }
}

/// Writes a 32-bit value to a memory-mapped I/O register
///
/// Performs a volatile write to the register at `base + offset`. The volatile
/// operation ensures the compiler won't optimize away the write.
///
/// # Safety
///
/// The caller must ensure:
/// - `base + offset` points to a valid, accessible 32-bit register
/// - The address is properly aligned for 32-bit access
/// - Writing this value won't cause undefined behavior or system instability
/// - The register is writable (not read-only)
pub unsafe fn write_mmio32(base: usize, offset: usize, value: u32) {
    unsafe {
        let ptr = (base as *mut u8).add(offset) as *mut u32;
        write_volatile(ptr, value);
    }
}
