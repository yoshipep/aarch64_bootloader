.section .text.boot

#include "asm/asmdefs.h"
#include "asm/boot.h"
#include "asm/macro.h"

###############################
# Bootloader entry point
# x0: The address of the dtb
###############################
ENTRY(_start)
	# Mask all interrupts except SError
	msr DAIFSet, #0x4
	# Set up stack
	adr x1, boot_stack
	mov sp, x1
	isb sy
	# Reserve stack space
	sub sp, sp, #16
	# Save the dtb so we can pass it later
	str x0, [sp, #0]
	# Load the interrupt vector for the bootloader. This bootloader is loaded at EL2
	# but I keep a generic approach by just checking CurrentEL
	switch_elx, x0, 1f, 2f, 3f
1:
	adr x0, evt
	msr VBAR_EL1, x0
	b set
2:
	adr x0,evt
	msr VBAR_EL2, x0
	b set
3:
	adr x0,evt
	msr VBAR_EL3, x0
set:
	isb sy
	# bl load_kernel
	# Pass the dtb and jump to kernel
	ldp x0, x4, [sp, #0]
	mov x1, #0x0
	mov x2, #0x0
	mov x3, #0x0
	br x4
	b .
END(_start)
