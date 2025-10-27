#include "asm/asmdefs.h"
#include "asm/boot.h"
#include "asm/macro.h"
#include "asm/system.h"

ENTRY(switch_to_elx)
	switch_elx x2, 1f, 2f, 3f
1:
	msr elr_el3, x1
	b end
2:
	msr elr_el2, x1
	mrs x2, hcr_el2
	/* Execution state for EL1 is AArch64 & Disable hypervisor calls */
	ldr x3, =(HCR_EL2_RW | HCR_EL2_HCD)
	orr x2, x2, x3
	msr hcr_el2, x2
	mrs x2, spsr_el2
	/* Mask all exception. Set execution state to AArch64 & Handle exceptions
	 * with SP_EL1 */
	ldr x3, =(SPSR_EL_DEBUG_MASK | SPSR_EL_SERR_MASK | SPSR_EL_IRQ_MASK | SPSR_EL_FIQ_MASK | \
			  SPSR_EL_M_AARCH64 | SPSR_EL_M_EL1)
	orr x2, x2, x3
	msr spsr_el2, x2
	b end
3:
	b .
end:
	eret
ENDPROC(switch_to_elx)
