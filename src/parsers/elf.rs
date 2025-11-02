//! ELF64 loader for AArch64
//!
//! This module provides functionality to parse and load 64-bit ELF (Executable
//! and Linkable Format) files. It validates the ELF header, processes program
//! headers, and loads executable segments into memory.
//!
//! The loader supports loading AArch64 executable files and returns the entry
//! point address for execution.

use crate::drivers::uart::pl011;

use core::{mem, ptr};

/// Size of the ELF magic number
const SELFMAG: usize = 4;
/// ELF magic number bytes: 0x7F 'E' 'L' 'F'
const ELFMAG: [u8; SELFMAG] = [0x7f, 0x45, 0x4c, 0x46];
/// Index of the file class byte in e_ident
const EI_CLASS: usize = 4;
/// 64-bit object file class
const ELFCLASS64: usize = 2;
/// Index of the data encoding byte in e_ident
const EI_DATA: usize = 5;
/// 2's complement, little-endian encoding
const ELFDATA2LSB: usize = 1;
/// Index of the OS/ABI identification in e_ident
const EI_OSABI: usize = 7;
/// UNIX System V ABI
const ELFOSABI_SYSV: usize = 0;

/// Executable file type
const ET_EXEC: usize = 2;

/// ARM AArch64 architecture
const EM_AARCH64: usize = 183;

/// Loadable program segment
const PT_LOAD: usize = 1;

/// ELF64 File Header
///
/// This structure represents the header of a 64-bit ELF file, containing
/// essential information about the file format, architecture, and layout.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Elf64Ehdr {
    /// ELF identification bytes (magic number, class, encoding, etc.)
    e_ident: [u8; 16],
    /// Object file type (e.g., ET_EXEC for executable)
    e_type: u16,
    /// Target architecture (e.g., EM_AARCH64)
    e_machine: u16,
    /// ELF format version
    e_version: u32,
    /// Virtual address of the program entry point
    e_entry: u64,
    /// File offset of the program header table
    e_phoff: u64,
    /// File offset of the section header table
    e_shoff: u64,
    /// Processor-specific flags
    e_flags: u32,
    /// Size of this ELF header in bytes
    e_ehsize: u16,
    /// Size of each program header table entry
    e_phentsize: u16,
    /// Number of program header table entries
    e_phnum: u16,
    /// Size of each section header table entry
    e_shentsize: u16,
    /// Number of section header table entries
    e_shnum: u16,
    /// Index of the section header string table
    e_shstrndx: u16,
}

/// ELF64 Program Header
///
/// This structure describes a segment or other information the system needs
/// to prepare the program for execution. Each loadable segment is copied
/// from the ELF file to memory at the specified virtual address.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Elf64Phdr {
    /// Type of segment (e.g., PT_LOAD for loadable segment)
    p_type: u32,
    /// Segment flags (read/write/execute permissions)
    p_flags: u32,
    /// Offset of the segment in the file
    p_offset: u64,
    /// Virtual address where segment should be loaded
    p_vaddr: u64,
    /// Physical address (for systems with physical addressing)
    p_paddr: u64,
    /// Size of the segment in the file
    p_filesz: u64,
    /// Size of the segment in memory (may be larger for BSS)
    p_memsz: u64,
    /// Segment alignment (0 and 1 mean no alignment)
    p_align: u64,
}

/// Loads an ELF kernel image from memory
///
/// Main entry point for loading a kernel. It parses the ELF file
/// at the given base address and loads it into memory.
#[unsafe(no_mangle)]
pub extern "C" fn load_kernel(elf_base: usize) -> usize {
    return load_elf(elf_base);
}

/// Validates an ELF64 header
///
/// Checks that the ELF header has the correct magic number,
/// is a 64-bit little-endian executable for AArch64.
fn check_elf_header(header: &Elf64Ehdr) -> bool {
    // Validate Magic
    if header.e_ident[0..4] != ELFMAG {
        pl011::println(b"Not an ELF file!");
        return false;
    }

    // Validate Bitness
    if header.e_ident[EI_CLASS] != ELFCLASS64 as u8 {
        pl011::println(b"Not an ELF file!");
        return false;
    }

    // Validate Endianess
    if header.e_ident[EI_DATA] != ELFDATA2LSB as u8 {
        pl011::println(b"Invalid endianess!");
        return false;
    }

    // Validate Class
    if header.e_ident[EI_OSABI] != ELFOSABI_SYSV as u8 {
        pl011::println(b"Invalid class!");
        return false;
    }

    // Validate Type
    if header.e_type != ET_EXEC as u16 {
        pl011::println(b"Invalid type!");
        return false;
    }

    // Validate Machine
    if header.e_machine != EM_AARCH64 as u16 {
        pl011::println(b"Invalid machine!");
        return false;
    }

    return true;
}

/// Loads an ELF file into memory from the given base address
///
/// Performs the complete ELF loading process:
/// 1. Validates the ELF header
/// 2. Iterates through all program headers
/// 3. Loads PT_LOAD segments to their target virtual addresses
/// 4. Zeros out BSS sections (when p_memsz > p_filesz)
fn load_elf(elf_base: usize) -> usize {
    let phdr_base;
    let header = unsafe { &*(elf_base as *const Elf64Ehdr) };

    // Validate ELF
    if !check_elf_header(header) {
        pl011::println(b"Not an ELF file!");
        panic!();
    }

    // Parse program headers
    phdr_base = elf_base + header.e_phoff as usize;
    for i in 0..header.e_phnum {
        let phdr = unsafe {
            &*((phdr_base + i as usize * mem::size_of::<Elf64Phdr>()) as *const Elf64Phdr)
        };

        if phdr.p_type == PT_LOAD as u32 {
            // PT_LOAD
            // Copy segment from ELF to target address
            let src = elf_base + phdr.p_offset as usize;
            let dst = phdr.p_vaddr as usize;
            let size = phdr.p_filesz as usize;
            unsafe {
                ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, size);
            }

            // Zero out BSS if memsz > filesz
            if phdr.p_memsz > phdr.p_filesz {
                let bss_start = dst + size;
                let bss_size = (phdr.p_memsz - phdr.p_filesz) as usize;
                unsafe {
                    ptr::write_bytes(bss_start as *mut u8, 0, bss_size);
                }
            }
        }
    }

    return header.e_entry as usize;
}
