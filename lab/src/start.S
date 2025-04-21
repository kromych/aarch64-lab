	.arch armv8-a
	.file	"start.S"
    .extern start, _stack_bot, _image_size, relocate, exception_handler
	.weak _DYNAMIC
	.hidden _DYNAMIC

	.macro EXCEPTION_ENTRY source, kind
	.align 7

		/* Volatile registers in AAPSC64 */
		ldp x0, x1, [sp]
		ldp x2, x3, [sp, #(2*8)]
		ldp x4, x5, [sp, #(4*8)]
		ldp x6, x7, [sp, #(6*8)]
		ldp x8, x9, [sp, #(8*8)]
		ldp x10, x11, [sp, #(10*8)]
		ldp x12, x13, [sp, #(12*8)]
		ldp x14, x15, [sp, #(14*8)]
		ldp x16, x17, [sp, #(16*8)]

		/* The linkage register */
		ldr x18, [sp, #(17*8)]

		/* Non-volatile registers in AAPSC64 */
		ldp x19, x20, [sp, #(19*8)]
		ldp x21, x22, [sp, #(21*8)]
		ldp x23, x24, [sp, #(23*8)]
		ldp x25, x26, [sp, #(25*8)]
		ldp x27, x28, [sp, #(27*8)]
		ldp x29, x30, [sp, #(29*8)]

		mov     x0, \source
		mov		x1, \kind
		/* Load the pointer to the exception frame into x3 */
		mov 	x3, sp
		sub 	x3, x3, 31*8

		bl exception_handler

		/* Volatile registers in AAPSC64 */
		stp x0, x1, [sp]
		stp x2, x3, [sp, #(2*8)]
		stp x4, x5, [sp, #(4*8)]
		stp x6, x7, [sp, #(6*8)]
		stp x8, x9, [sp, #(8*8)]
		stp x10, x11, [sp, #(10*8)]
		stp x12, x13, [sp, #(12*8)]
		stp x14, x15, [sp, #(14*8)]
		stp x16, x17, [sp, #(16*8)]

		/* The linkage register */
		str x18, [sp, #(17*8)]

		/* Non-volatile registers in AAPSC64 */
		stp x19, x20, [sp, #(19*8)]
		stp x21, x22, [sp, #(21*8)]
		stp x23, x24, [sp, #(23*8)]
		stp x25, x26, [sp, #(25*8)]
		stp x27, x28, [sp, #(27*8)]
		stp x29, x30, [sp, #(29*8)]

		eret
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
