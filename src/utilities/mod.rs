//! Utility functions and helpers
//!
//! This module contains common utility functions that are used across multiple
//! parts of the bootloader. These utilities provide low-level functionality
//! that doesn't fit into specific driver or parser modules.
//!
//! # Available Utilities
//!
//! - [`mmio`]: Memory-mapped I/O operations
//!   - Safe wrappers for volatile memory reads and writes
//!   - Bit manipulation helpers (set/clear bits)
//!   - Used by hardware drivers to access device registers
//!
//! - [`print`]: Hexadecimal printing utilities
//!   - Format and print u64 and u8 values in hexadecimal
//!   - Used by exception handlers for debugging output
//!   - Operates directly on UART without requiring formatting traits

pub mod mmio;
pub mod print;
