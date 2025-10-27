#include "asm/boot.h"

OUTPUT_ARCH("aarch64")

OUTPUT_FORMAT("elf64-littleaarch64")

ENTRY(_start)

MEMORY {
    RAM : ORIGIN = 0x40080000, LENGTH = 1M
}

SECTIONS {
    . = ORIGIN(RAM);
    __bootloader_start = .;

    .text : {
        __text_start = .;
        *(.text.boot)
        *(.text*)
        __text_end = .;
    } > RAM

    __data_start = .;
    .rodata : ALIGN(8) {
        __rodata_start = .;
        *(.rodata*)
        __rodata_end = .;
    } > RAM

    .data : ALIGN(8) {
        *(.data*)
    } > RAM

    /* Mark end of initialized data - this is where binary file ends */
    __binary_end = .;

    .bss : ALIGN(8) {
        _bss_start = .;
        *(.bss*)
        *(COMMON)
        _bss_end = .;
    } > RAM

    __data_end = .;

    __bootloader_end = .;
    . = . + BOOT_STACK_SIZE;
    . = ALIGN(4K);
    boot_stack = .;
}
