#include "asm/boot.h"

OUTPUT_FORMAT("elf64-littleaarch64")

OUTPUT_ARCH("aarch64")

ENTRY(_start)

MEMORY {
    RAM : ORIGIN = 0x40080000, LENGTH = 1M
}

SECTIONS {
    . = ORIGIN(RAM);

    .text : {
        *(.text.boot)
        *(.text*)
    } > RAM

    .rodata : ALIGN(8) {
        *(.rodata*)
    } > RAM

    .data : ALIGN(8) {
        *(.data*)
    } > RAM

    .bss : ALIGN(8) {
        _bss_start = .;
        *(.bss*)
        *(COMMON)
        _bss_end = .;
    } > RAM

    . = . + BOOT_STACK_SIZE;
    . = ALIGN(4K);
    boot_stack = .;
    _bootloader_end = .;
}
