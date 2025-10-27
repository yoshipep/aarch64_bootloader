#include "asm/macro.h"

.global evt
.align 11

evt:
.align 7 /* Current EL Synchronous */
	bl save_regs
	bl do_bad_sync
	b exception_exit

.align 7 /* Current EL IRQ */
	bl save_regs
	bl do_bad_irq
	b exception_exit

.align 7 /* Current EL FIQ */
	bl save_regs
	bl do_bad_fiq
	b exception_exit

.align 7 /* Current EL SError */
	bl save_regs
	bl do_bad_serror
	b exception_exit

.align 7 /* Current EL SP_ELx Synchronous */
	bl save_regs
	bl do_sync
	b exception_exit

.align 7 /* Current EL SP_ELx IRQ */
	bl save_regs
	bl do_irq
	b exception_exit

.align 7 /* Current EL SP_ELx FIQ */
	bl save_regs
	bl do_fiq
	b exception_exit

.align 7 /* Current EL SP_ELx SError */
	bl save_regs
	bl do_serror
	b exception_exit

save_regs:
	alloc_stack 288
	saveregs
	switch_elx x0, 1f, 2f, 3f
1:
	mrs x1, esr_el1
	mrs x2, elr_el1
	mrs x3, spsr_el1
	b 0f
2:
	mrs x1, esr_el2
	mrs x2, elr_el2
	mrs x3, spsr_el2
	b 0f
3:
	mrs x1, esr_el3
	mrs x2, elr_el3
	mrs x3, spsr_el3
0:
	stp x1, x2, [sp, #256]
	stp x3, xzr, [sp, #272]
	mov x0, sp
	ret

exception_exit:
	ldp x0, xzr, [sp, #256]
	switch_elx x0, 1f, 2f, 3f
1:
	msr elr_el1, x1
	b 0f
2:
	msr elr_el2, x1
	b 0f
3:
	msr elr_el3, x1
0:
	restoreregs
	dealloc_stack 288
	eret
