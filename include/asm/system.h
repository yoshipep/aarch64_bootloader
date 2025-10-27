#ifndef SYSTEM_H_
#define SYSTEM_H_

/* SCTLR_ELx bits */
#define CR_MMU (1 << 0)
#define CR_DCACHE (1 << 2)
#define CR_ICACHE (1 << 12)

/* HCR_EL2 bits */
#define HCR_EL2_HCD (1 << 29)
#define HCR_EL2_RW (1 << 31)

/* SPSR_ELx bits */
// Asm doesnt like ~SPSR_EL_M_AARCH64
#define SPSR_EL_M_AARCH64 (0 << 4)
#define SPSR_EL_FIQ_MASK (1 << 6)
#define SPSR_EL_IRQ_MASK (1 << 7)
#define SPSR_EL_SERR_MASK (1 << 8)
#define SPSR_EL_DEBUG_MASK (1 << 9)
#define SPSR_EL_M_EL1 (5)

#endif // SYSTEM_H_
