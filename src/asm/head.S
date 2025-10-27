.section .text.boot

#include "asm/asmdefs.h"
#include "asm/boot.h"
#include "asm/macro.h"

.equ UART_BASE_ADDR, 0x09000000
.equ UART_CLOCK_FREQ, 0x16e3600
.equ UART_BAUD_RATE, 0x1c0200

/*
* Bootloader entry point
* x0: The address of the dtb
*/
ENTRY(_start)
	/* Mask all interrupts except SError */
	msr DAIFSet, #0x4
	/* Set up stack */
	adr x1, boot_stack
	mov sp, x1
	isb sy
	/* Reserve stack space (dtb addr + padding for alignement) */
	sub sp, sp, #16
	/* Save the dtb so we can pass it later */
	str x0, [sp, #0]
	adr x1, evt
	/* Load the interrupt vector for the bootloader. This bootloader is loaded at EL2
	 * but I keep a generic approach by just checking CurrentEL
	 */
	switch_elx x0, 1f, 2f, 3f
1:
	msr VBAR_EL1, x1
	b set
2:
	msr VBAR_EL2, x1
	b set
3:
	msr VBAR_EL3, x1
set:
	isb sy
	/* Setup early UART for printing */
	ldr x0, =UART_BASE_ADDR
	ldr w1, =UART_CLOCK_FREQ
	ldr w2, =UART_BAUD_RATE
	bl init_uart
	bl configure_uart
	/* Calculate kernel ELF address. Kernel starts immediately after
	 * bootloader binary
	 */
	adr x1, __bootloader_end
	adr x2, __bootloader_start
	sub x1, x1, x2
	add x0, x1, x2
	mov x1, #KERNEL_ALIGN
	add x0, x0, x1               /* Start + one alignment unit */
	neg x1, x1
	and x0, x0, x1               /* Round up to aligned boundary */
	bl load_kernel
	mov x1, x0
	/* Pass the dtb and jump to kernel */
	ldp x0, xzr, [sp, #0]
	bl switch_to_elx
ENDPROC(_start)
