//! Exception handling module

/// Struct that represents the machine state
///
/// This struct is used to store the registers of the cpu in a context switch
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
}

#[unsafe(no_mangle)]
pub extern "C" fn do_bad_sync(regs: *const Regs) {
    panic!();
}

#[unsafe(no_mangle)]
pub fn do_bad_irq(regs: *const Regs) -> u32 {
    panic!();
}

#[unsafe(no_mangle)]
pub extern "C" fn do_bad_fiq(regs: *const Regs) -> ! {
    panic!();
}

#[unsafe(no_mangle)]
pub extern "C" fn do_bad_serror(regs: *const Regs) -> ! {
    panic!();
}

#[unsafe(no_mangle)]
pub extern "C" fn do_sync(regs: *const Regs) {
    panic!();
}

#[unsafe(no_mangle)]
pub fn do_irq(regs: *const Regs) -> u32 {
    panic!();
}

#[unsafe(no_mangle)]
pub extern "C" fn do_fiq(regs: *const Regs) -> ! {
    panic!();
}

#[unsafe(no_mangle)]
pub extern "C" fn do_serror(regs: *const Regs) -> ! {
    panic!();
}
