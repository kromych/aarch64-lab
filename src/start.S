	.arch armv8-a
	.file	"start.S"
    .extern main, _stack_bot, _image_size, relocate
	.weak _DYNAMIC
	.hidden _DYNAMIC

	.macro EXCEPTION_ENTRY source, kind
	.align 7
		mov     x20, \source
		mov		x21, \kind
		mrs     x22, FAR_EL1
		mrs     x23, ESR_EL1
		mrs     x24, ELR_EL1
		b		.
	.endm

	.global	_start

	.section ".init.text", "ax", @progbits

	.align	12
	.type	_start, %function

	// This resembles the Linux kernel Image header

	add     x13, x18, #0x16  // code0: UEFI "MZ" signature magic instruction
	bl      _start
	.quad	0				// Image load offset from start of RAM, little-endian
	.quad	_image_size		// Kernel size, little-endian
	.quad	0x0A			// Flags, little-endian: LE kernel(bit 0), 4K pages(bits 1-2), phys addr(bit 3)
	.quad	0				// Reserved
	.quad	0				// Reserved
	.quad	0				// Reserved
	.ascii	"ARM\x64"		// Signature

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

    // NEON setup. To be exactly correct need to see
	// what the current EL is.
    mov 	x1, #(0x3 << 20)
    msr 	CPACR_EL1, x1
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

	bl      main
	b       .
