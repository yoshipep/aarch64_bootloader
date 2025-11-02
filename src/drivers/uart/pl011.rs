//! UART PL011 driver for AArch64
//!
//! This module provides a minimal driver for the ARM PL011 UART device,
//! commonly used in ARM development boards and QEMU. It handles initialization,
//! configuration, and basic character output via memory-mapped I/O (MMIO).
//!
//! The driver supports configurable baud rates, data bits, and stop bits.

use crate::utilities::mmio;
use core::ptr::{null_mut, write_volatile};

/// UART PL011 device configuration
///
/// This struct holds the configuration for a PL011 UART device, including
/// the memory-mapped base address and communication parameters.
struct UartPl011 {
    /// Memory-mapped base address of the UART device
    base_addr: *mut u32,
    /// Base clock frequency in Hz (used for baud rate calculation)
    base_clock: u32,
    /// Target baud rate (bits per second)
    baudrate: u32,
    /// Number of data bits per frame (typically 8)
    data_bits: u8,
    /// Number of stop bits (1 or 2)
    stop_bits: u8,
}

// PL011 Register Offsets
/// Data Register offset - used for reading/writing data
const DR_OFF: usize = 0x00;
/// Flag Register offset - contains status flags
const FR_OFF: usize = 0x18;
/// Flag Register BUSY bit - indicates UART is transmitting
const FR_BUSY: u32 = 1 << 3;
/// Integer Baud Rate Divisor Register offset
const IBRD_OFF: usize = 0x24;
/// Fractional Baud Rate Divisor Register offset
const FBRD_OFF: usize = 0x28;
/// Line Control Register offset - configures data format
const LCR_OFF: usize = 0x2c;
/// Line Control Register FIFO Enable bit
const LCR_FEN: u32 = 1 << 4;
/// Line Control Register 2 Stop Bits bit
const LCR_STP2: u32 = 1 << 3;
/// Control Register offset - enables/disables UART and TX/RX
const CR_OFF: usize = 0x30;
/// Control Register UART Enable bit
const CR_UARTEN: u32 = 1 << 0;
/// Control Register Transmit Enable bit
const CR_TXEN: u32 = 1 << 8;
/// Interrupt Mask Set/Clear Register offset
const IMSC_OFF: usize = 0x38;
/// DMA Control Register offset
const DMACR_OFF: usize = 0x48;

/// Global UART device instance
static mut UART: UartPl011 = UartPl011 {
    base_addr: null_mut(),
    base_clock: 0,
    baudrate: 0,
    data_bits: 0,
    stop_bits: 0,
};

/// Initializes the global UART device with the given parameters
///
/// This function must be called before any UART operations. It sets up the
/// UART configuration with 8 data bits and 1 stop bit by default.
#[unsafe(no_mangle)]
pub fn init_uart(base_addr: *mut u32, base_clock: u32, baudrate: u32) {
    unsafe {
        UART = UartPl011 {
            base_addr: base_addr,
            base_clock: base_clock,
            baudrate: baudrate,
            data_bits: 8,
            stop_bits: 1,
        };
    }
}

/// Configures the UART device according to the initialized parameters
///
/// Performs the complete configuration sequence for the PL011 UART:
/// 1. Disables the UART
/// 2. Waits for any pending transmissions to complete
/// 3. Flushes the TX FIFO
/// 4. Sets the baud rate
/// 5. Configures the data frame format (data bits, stop bits)
/// 6. Masks all interrupts
/// 7. Disables DMA
/// 8. Enables transmission
/// 9. Re-enables the UART
#[unsafe(no_mangle)]
pub fn configure_uart() {
    let mut cfg: u32;
    // 1. Disable the UART
    unsafe {
        mmio::write_mmio32(UART.base_addr as usize, CR_OFF, CR_UARTEN);
    }
    // 2. Wait for the end of TX
    loop {
        if uart_ready() {
            break;
        }
    }
    // 3. Flush TX FIFO
    unsafe {
        mmio::write_mmio32(UART.base_addr as usize, LCR_OFF, !LCR_FEN);
    }
    // 4. Set speed
    uart_set_speed();
    // 5. Configure the data frame format
    // 5.1 Word length: bits 5 and 6
    cfg = 0;
    unsafe {
        cfg |= ((UART.data_bits as u32 - 1) & 0x3) << 5;
        // 5.2 Use 1 or 2 stop bits: bit LCR_STP2
        if UART.stop_bits == 2 {
            cfg |= LCR_STP2;
        }
        mmio::write_mmio32(UART.base_addr as usize, LCR_OFF, cfg);
    }
    // 6. Mask all interrupts
    unsafe {
        mmio::write_mmio32(UART.base_addr as usize, IMSC_OFF, 0x0);
    // 7. Disable DMA
        mmio::write_mmio32(UART.base_addr as usize, DMACR_OFF, 0x0);
    // 8. Enable TX and UART
        mmio::write_mmio32(UART.base_addr as usize, CR_OFF, CR_TXEN | CR_UARTEN);
    }
}

/// Checks if the UART is ready for transmission
#[inline(always)]
fn uart_ready() -> bool {
    unsafe {
        return (mmio::read_mmio32(UART.base_addr as usize, FR_OFF) & FR_BUSY) == 0;
    }
}

/// Configures the UART baud rate based on the base clock and desired baudrate
///
/// The baud rate divisor is calculated as: `(4 * base_clock) / baudrate`
/// The integer part is written to IBRD and the fractional part to FBRD.
fn uart_set_speed() {
    let baud_div;

    unsafe {
        baud_div = 4 * UART.base_clock / UART.baudrate;
        mmio::write_mmio32(UART.base_addr as usize, IBRD_OFF, (baud_div >> 6) & 0xffff);
        mmio::write_mmio32(UART.base_addr as usize, FBRD_OFF, baud_div & 0x3f);
    }
}

/// Transmits a single character via UART
///
/// Waits until the UART is ready before writing the character
/// to the data register.
fn putchar(c: u8) {
    let addr;

    loop {
        if uart_ready() {
            break;
        }
    }
    unsafe {
        addr = UART.base_addr as *mut u8;
        write_volatile(addr.add(DR_OFF), c);
    }
}

/// Prints a byte slice to the UART
pub fn print(s: &[u8]) {
    for &c in s {
        putchar(c);
    }
}

/// Prints a byte slice followed by a newline to the UART
pub fn println(s: &[u8]) {
    print(s);
    putchar(b'\n');
}
