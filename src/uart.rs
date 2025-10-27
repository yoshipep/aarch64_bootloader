use core::ptr::{null_mut, read_volatile, write_volatile};

// Define the address of the UART device (MMIO)
// In C: volatile uint8_t *uart = ...
struct UartPl011 {
    base_addr: *mut u32,
    base_clock: u32,
    baudrate: u32,
    data_bits: u8,
    stop_bits: u8,
}

const DR_OFF: usize = 0x00;
const FR_OFF: usize = 0x18;
const FR_BUSY: u32 = 1 << 3;
const IBRD_OFF: usize = 0x24;
const FBRD_OFF: usize = 0x28;
const LCR_OFF: usize = 0x2c;
const LCR_FEN: u32 = 1 << 4;
const LCR_STP2: u32 = 1 << 3;
const CR_OFF: usize = 0x30;
const CR_UARTEN: u32 = 1 << 0;
const CR_TXEN: u32 = 1 << 8;
const IMSC_OFF: usize = 0x38;
const DMACR_OFF: usize = 0x48;

static mut UART: UartPl011 = UartPl011 {
    base_addr: null_mut(),
    base_clock: 0,
    baudrate: 0,
    data_bits: 0,
    stop_bits: 0,
};

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

fn read_reg(offset: usize) -> u32 {
    unsafe {
        let ptr = (UART.base_addr as *const u8).add(offset) as *const u32;
        return read_volatile(ptr);
    }
}

fn write_reg(offset: usize, val: u32) {
    unsafe {
        let mut contents = read_reg(offset) as u32;
        // Mask to save the bits that we are not overwriting
        contents &= !val;
        contents |= val;
        write_volatile(
            (UART.base_addr as *mut u8).add(offset) as *mut u32,
            contents,
        );
    }
}

#[unsafe(no_mangle)]
pub fn configure_uart() {
    let mut cfg: u32;
    // 1. Disable the UART
    disable_uart();
    // 2. Wait for the end of TX
    loop {
        if uart_ready() {
            break;
        }
    }
    // 3. Flush TX FIFO
    uart_flush_tx_fifo();
    // 4. Set speed
    uart_set_speed();
    // 5. Configure the data frame format
    // 5.1 Word length: bits 5 and 6
    cfg = 0;
    unsafe {
        cfg |= (((UART.data_bits - 1) & 0x3) << 5) as u32;
        // 5.2 Use 1 or 2 stop bits: bit LCR_STP2
        if UART.stop_bits == 2 {
            cfg |= LCR_STP2;
        }
    }
    uart_write_lcr(cfg);
    // 6. Mask all interrupts
    uart_write_msc(0x7ff);
    // 7. Disable DMA
    uart_write_dmacr(0);
    // 8. Enable TX
    uart_write_cr(CR_TXEN);
    // 9. Enable UART
    uart_write_cr(CR_UARTEN);
}

fn disable_uart() {
    uart_write_cr(CR_UARTEN);
}

fn uart_ready() -> bool {
    return (read_reg(FR_OFF) & FR_BUSY) == 0;
}

fn uart_flush_tx_fifo() {
    write_reg(LCR_OFF, LCR_FEN);
}

fn uart_write_lcr(cfg: u32) {
    write_reg(LCR_OFF, cfg);
}

fn uart_write_cr(cfg: u32) {
    write_reg(CR_OFF, cfg);
}

fn uart_write_msc(mask: u32) {
    write_reg(IMSC_OFF, mask);
}

fn uart_write_dmacr(mask: u32) {
    write_reg(DMACR_OFF, mask);
}

fn uart_set_speed() {
    let baud_div;

    unsafe {
        baud_div = 4 * UART.base_clock / UART.baudrate;
        write_reg(IBRD_OFF, (baud_div >> 6) & 0xffff);
        write_reg(FBRD_OFF, baud_div & 0x3f);
    }
}

fn putchar(c: u8) {
    loop {
        if uart_ready() {
            break;
        }
    }
    write_reg(DR_OFF, c as u32);
}

pub fn print(s: &[u8]) {
    for &c in s {
        putchar(c);
    }
}

pub fn println(s: &[u8]) {
    print(s);
    putchar(b'\n');
}
