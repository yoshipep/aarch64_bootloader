use crate::uart;

use core::{mem, ptr};

const SELFMAG: usize = 4;
const ELFMAG: [u8; SELFMAG] = [0x7f, 0x45, 0x4c, 0x46];

const EI_CLASS: usize = 4; /* File class byte index */
const ELFCLASS64: usize = 2; /* 64-bit objects */

const EI_DATA: usize = 5; /* Data encoding byte index */
const ELFDATA2LSB: usize = 1; /* 2's complement, little endian */

const EI_OSABI: usize = 7; /* OS ABI identification */
const ELFOSABI_SYSV: usize = 0; /* Alias.  */

const ET_EXEC: usize = 2; /* Executable file */

const EM_AARCH64: usize = 183; /* ARM AARCH64 */

const PT_LOAD: usize = 1; /* Loadable program segment */

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Elf64Ehdr {
    e_ident: [u8; 16], /* Magic number and other info */
    e_type: u16,       /* Object file type */
    e_machine: u16,    /* Architecture */
    e_version: u32,    /* Object file version */
    e_entry: u64,      /* Entry point virtual address */
    e_phoff: u64,      /* Program header table file offset */
    e_shoff: u64,      /* Section header table file offset */
    e_flags: u32,      /* Processor-specific flags */
    e_ehsize: u16,     /* ELF header size in bytes */
    e_phentsize: u16,  /* Program header table entry size */
    e_phnum: u16,      /* Program header table entry count */
    e_shentsize: u16,  /* Section header table entry size */
    e_shnum: u16,      /* Section header table entry count */
    e_shstrndx: u16,   /* Section header string table index */
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Elf64Phdr {
    p_type: u32,   /* Segment type */
    p_flags: u32,  /* Segment flags */
    p_offset: u64, /* Segment file offset */
    p_vaddr: u64,  /* Segment virtual address */
    p_paddr: u64,  /* Segment physical address */
    p_filesz: u64, /* Segment size in file */
    p_memsz: u64,  /* Segment size in memory */
    p_align: u64,  /* Segment alignment */
}

#[unsafe(no_mangle)]
pub extern "C" fn load_kernel(elf_base: usize) -> usize {
    return load_elf(elf_base);
}

fn check_elf_header(header: &Elf64Ehdr) -> bool {
    // Validate Magic
    if header.e_ident[0..4] != ELFMAG {
        uart::println(b"Not an ELF file!");
        return false;
    }

    // Validate Bitness
    if header.e_ident[EI_CLASS] != ELFCLASS64 as u8 {
        uart::println(b"Not an ELF file!");
        return false;
    }

    // Validate Endianess
    if header.e_ident[EI_DATA] != ELFDATA2LSB as u8 {
        uart::println(b"Invalid endianess!");
        return false;
    }

    // Validate Class
    if header.e_ident[EI_OSABI] != ELFOSABI_SYSV as u8 {
        uart::println(b"Invalid class!");
        return false;
    }

    // Validate Type
    if header.e_type != ET_EXEC as u16 {
        uart::println(b"Invalid type!");
        return false;
    }

    // Validate Machine
    if header.e_machine != EM_AARCH64 as u16 {
        uart::println(b"Invalid machine!");
        return false;
    }

    return true;
}

fn load_elf(elf_base: usize) -> usize {
    let phdr_base;
    let header = unsafe { &*(elf_base as *const Elf64Ehdr) };

    // Validate ELF
    if !check_elf_header(header) {
        uart::println(b"Not an ELF file!");
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

    return header.e_entry as usize; // Return entry point
}
