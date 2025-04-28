	.arch armv8-a
	.file	"start.S"
    .extern start, _stack_bot, _image_size, relocate, exception_handler
	.weak _DYNAMIC
	.hidden _DYNAMIC

.section .text

/*
 * Good only for the EL1.
 * Refer to AAPSC64 for the details on volatile and non-volatile registers.
 */
__exception_common:
	str     x29, [sp, #-16]!
	stp     x27, x28, [sp, #-16]!
	stp     x25, x26, [sp, #-16]!
	stp     x23, x24, [sp, #-16]!
	stp     x21, x22, [sp, #-16]!
	stp     x19, x20, [sp, #-16]!
	stp     x17, x18, [sp, #-16]!
	stp     x15, x16, [sp, #-16]!
	stp     x13, x14, [sp, #-16]!
	stp     x11, x12, [sp, #-16]!
	stp     x9, x10, [sp, #-16]!
	stp     x7, x8, [sp, #-16]!
	stp     x5, x6, [sp, #-16]!
	stp     x3, x4, [sp, #-16]!
	stp     x1, x2, [sp, #-16]!

	add     sp, sp, #-16

	mrs     x2, spsr_el1
	mrs     x1, elr_el1
	stp     x1, x2, [sp, #-16]!

	str     x0, [sp, #-16]!

	mrs     x2, tpidr_el1
	add     x1, sp, #38*8
	stp     x1, x2, [sp, #32]

	mov     x0, sp
	bl      exception_handler

	ldr     x1, [sp, #40]
	msr     tpidr_el1, x1

	add     sp, sp, #16

	ldp     x1, x2, [sp], #16
	msr     elr_el1, x1
	msr     spsr_el1, x2

	add     sp, sp, #16

	ldp     x1, x2, [sp], #16
	ldp     x3, x4, [sp], #16
	ldp     x5, x6, [sp], #16
	ldp     x7, x8, [sp], #16
	ldp     x9, x10, [sp], #16
	ldp     x11, x12, [sp], #16
	ldp     x13, x14, [sp], #16
	ldp     x15, x16, [sp], #16
	ldp     x17, x18, [sp], #16
	ldp     x19, x20, [sp], #16
	ldp     x21, x22, [sp], #16
	ldp     x23, x24, [sp], #16
	ldp     x25, x26, [sp], #16
	ldp     x27, x28, [sp], #16
	ldr     x29, [sp], #16
	ldp     lr, x0, [sp], #16

	eret

	.macro EXCEPTION_ENTRY source, kind
	.align 7
		stp     lr, x0, [sp, #-16]!
		mov     x0, \source
		movk    x0, \kind, lsl #16
		b       __exception_common
	.endm

	.global	_start

	.section ".init.text", "ax", @progbits

	.align	12
	.type	_start, %function

	// This resembles the Linux kernel Image header

	add     x13, sp, #0x16  // code0: UEFI "MZ" signature magic instruction
	bl      _start
	.quad	0				// Image load offset from start of RAM, little-endian
	.quad	_image_size		// Kernel size, little-endian
	.quad	0x0A			// Flags, little-endian: LE kernel(bit 0), 4K pages(bits 1-2), phys addr(bit 3)
	.quad	0				// Reserved
	.quad	0				// Reserved
	.quad	0				// Reserved
	.ascii	"ARM\x64"		// Signature

    // Vector table must be aligned to a 2KB boundary
	.balign 0x800
_vector_table_el1:
    // Target and source at same exception level with source SP = SP_EL0
    EXCEPTION_ENTRY #0x0, #0x0  // Synchronous exception
    EXCEPTION_ENTRY #0x0, #0x1  // IRQ
    EXCEPTION_ENTRY #0x0, #0x2  // FIQ
    EXCEPTION_ENTRY #0x0, #0x3  // SError

    // Target and source at same exception level with source SP = SP_ELx
    EXCEPTION_ENTRY #0x1, #0x0  // Synchronous exception
    EXCEPTION_ENTRY #0x1, #0x1  // IRQ
    EXCEPTION_ENTRY #0x1, #0x2  // FIQ
    EXCEPTION_ENTRY #0x1, #0x3  // SError

    // Source is at lower exception level running on AArch64
    EXCEPTION_ENTRY #0x2, #0x0  // Synchronous exception
    EXCEPTION_ENTRY #0x2, #0x1  // IRQ
    EXCEPTION_ENTRY #0x2, #0x2  // FIQ
    EXCEPTION_ENTRY #0x2, #0x3  // SError

    // Source is at lower exception level running on AArch32
    EXCEPTION_ENTRY #0x3, #0x0  // Synchronous exception
    EXCEPTION_ENTRY #0x3, #0x1  // IRQ
    EXCEPTION_ENTRY #0x3, #0x2  // FIQ
    EXCEPTION_ENTRY #0x3, #0x3  // SError

_start:
    adrp 	x3, _vector_table_el1
    add 	x3, x3, :lo12:_vector_table_el1
    msr     VBAR_EL1, x3
    isb

    // NEON and FP setup. To be exactly correct need to see
	// what the current EL is.
	mrs	x0, CPACR_EL1
	orr	x0, x0, #(3 << 20)
	orr	x0, x0, #(3 << 16)
	msr	CPACR_EL1, x0
    isb

	// Set up the stack
    adrp    x1, _stack_bot
	sub		x1, x1, 64
    mov     sp, x1

    // Rust compiler produces globals for formatting calls,
    // need to relocate.

    adrp 	x0, _base
    adrp 	x1, _DYNAMIC
    add 	x1, x1, :lo12:_DYNAMIC
	bl 		relocate

	// Run the Rust main

	bl      start
	b       .

	.section ".bss.page_tables", "aw", @nobits
    .global _page_tables_start
    .global _page_tables_end

    .balign 0x4000
_page_tables_start:
    .space 0x800000
_page_tables_end:
