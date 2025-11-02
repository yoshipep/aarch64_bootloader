//! Exception handling for AArch64
//!
//! This module provides exception handlers for the AArch64 architecture,
//! including synchronous exceptions, IRQ, FIQ, and SError handlers. When
//! an exception occurs, the handlers print diagnostic information including
//! the faulting instruction and register state before panicking.
//!
//! The module supports both "bad mode" handlers (for unexpected exception
//! levels) and normal exception handlers.

use crate::utilities::print::{print_hex_u64, print_hex_u8};
use crate::drivers::uart::pl011;

/// CPU register state at the time of an exception
///
/// This struct captures all general-purpose registers (x0-x30) and special
/// system registers when an exception occurs. The layout matches the order
/// in which registers are saved by the exception entry code.
///
/// # Fields
///
/// - `x0-x30`: General-purpose registers
/// - `esr`: Exception Syndrome Register - describes the exception cause
/// - `elr`: Exception Link Register - return address
/// - `spsr`: Saved Program Status Register - saved processor state
/// - `zr`: Zero register placeholder
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Regs {
    x0: u64,
    x1: u64,
    x2: u64,
    x3: u64,
    x4: u64,
    x5: u64,
    x6: u64,
    x7: u64,
    x8: u64,
    x9: u64,
    x10: u64,
    x11: u64,
    x12: u64,
    x13: u64,
    x14: u64,
    x15: u64,
    x16: u64,
    x17: u64,
    x18: u64,
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    x28: u64,
    x29: u64,
    x30: u64,
    esr: u64,
    elr: u64,
    spsr: u64,
    zr: u64,
}

impl Regs {
    /// Register names for iteration
    const NAMES: [&'static str; 35] = [
        "x0 ", "x1 ", "x2 ", "x3 ", "x4 ", "x5 ", "x6 ", "x7 ", "x8 ", "x9 ", "x10", "x11", "x12",
        "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25",
        "x26", "x27", "x28", "x29", "x30", "esr", "elr", "spsr", "xzr",
    ];

    /// Convert registers to an array for easy iteration
    pub fn as_array(&self) -> [u64; 35] {
        [
            self.x0, self.x1, self.x2, self.x3, self.x4, self.x5, self.x6, self.x7, self.x8,
            self.x9, self.x10, self.x11, self.x12, self.x13, self.x14, self.x15, self.x16,
            self.x17, self.x18, self.x19, self.x20, self.x21, self.x22, self.x23, self.x24,
            self.x25, self.x26, self.x27, self.x28, self.x29, self.x30, self.esr, self.elr,
            self.spsr, self.zr,
        ]
    }

    /// Returns an iterator over (name, value) pairs for all registers
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, u64)> {
        self.as_array()
            .into_iter()
            .zip(Self::NAMES.iter().copied())
            .map(|(val, name)| (name, val))
    }

    /// Print all registers to UART
    pub fn print(&self) {
        pl011::println(b"\nRegisters:");
        for (name, value) in self.iter() {
            pl011::print(name.as_bytes());
            pl011::print(b": 0x");
            print_hex_u64(value);
            pl011::print(b"\n");
        }
    }
}

/// Prints the faulting instruction at the exception address
///
/// Reads and displays the 32-bit instruction at the address stored in the
/// Exception Link Register (ELR), which points to the instruction that
/// caused the exception.
fn print_faulting_instr(elr: u64) {
    let opcode: u32;
    let addr = (elr & !3) as *const u32;

    pl011::print(b"Faulting instruction at 0x");
    print_hex_u64(elr);
    pl011::print(b": ");
    unsafe {
        opcode = addr.read_volatile();
    }

    for i in 0..4 {
        if i == 0 {
            pl011::print(b"[");
            print_hex_u8((opcode >> (i * 8)) as u8);
            pl011::print(b"]")
        } else {
            print_hex_u8((opcode >> (i * 8)) as u8);
        }

        if i < 3 {
            pl011::print(b" ");
        }
    }

    pl011::print(b"\n");
}

/// Prints all CPU registers from the saved register state
fn print_regs(regs: *const Regs) {
    // Print register dump
    unsafe {
        if let Some(regs) = regs.as_ref() {
            regs.print();
        }
    }
}

/// Handles synchronous exceptions from an unexpected exception level
///
/// This "bad mode" handler is called when a synchronous exception occurs
/// from an exception level that should not normally generate exceptions.
/// It prints diagnostic information and panics.
#[unsafe(no_mangle)]
pub extern "C" fn do_bad_sync(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"Bad mode in Synchronous Exception handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles IRQ (Interrupt Request) from an unexpected exception level
///
/// This "bad mode" handler is called when an IRQ occurs from an exception
/// level that should not normally generate interrupts. It prints diagnostic
/// information and panics.
#[unsafe(no_mangle)]
pub extern "C" fn do_bad_irq(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"Bad mode in IRQ handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles FIQ (Fast Interrupt Request) from an unexpected exception level
///
/// This "bad mode" handler is called when an FIQ occurs from an exception
/// level that should not normally generate fast interrupts. It prints
/// diagnostic information and panics.
#[unsafe(no_mangle)]
pub extern "C" fn do_bad_fiq(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"Bad mode in FIQ handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles SError (System Error) from an unexpected exception level
///
/// This "bad mode" handler is called when a system error occurs from an
/// exception level that should not normally generate SErrors. It prints
/// diagnostic information and panics.
#[unsafe(no_mangle)]
pub extern "C" fn do_bad_serror(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"Bad mode in SError handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles synchronous exceptions from the current exception level
///
/// Called when a synchronous exception occurs (e.g., undefined instruction,
/// data abort, etc.). Prints diagnostic information including the faulting
/// instruction and register state, then panics.
#[unsafe(no_mangle)]
pub extern "C" fn do_sync(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"Synchronous Exception handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles IRQ (Interrupt Request) from the current exception level
///
/// Called when an interrupt request is received. Prints diagnostic
/// information and panics (as interrupt handling is not yet implemented).
#[unsafe(no_mangle)]
pub extern "C" fn do_irq(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"IRQ handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles FIQ (Fast Interrupt Request) from the current exception level
///
/// Called when a fast interrupt request is received. Prints diagnostic
/// information and panics (as FIQ handling is not yet implemented).
#[unsafe(no_mangle)]
pub extern "C" fn do_fiq(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"FIQ handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}

/// Handles SError (System Error) from the current exception level
///
/// Called when a system error occurs (e.g., asynchronous external abort).
/// Prints diagnostic information and panics.
#[unsafe(no_mangle)]
pub extern "C" fn do_serror(regs: *const Regs) -> ! {
    let elr;

    pl011::println(b"SError handler");
    unsafe {
        elr = (&*regs).elr;
    }
    print_faulting_instr(elr);
    print_regs(regs);
    panic!();
}
