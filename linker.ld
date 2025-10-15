ENTRY(_start)

MEMORY {
    RAM : ORIGIN = 0x40000000, LENGTH = 1M
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

    _bootloader_end = .;
    . = . + 4K;
    . = ALIGN(16);
    stack_top = .;
}
