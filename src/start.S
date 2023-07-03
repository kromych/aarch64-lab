	.arch armv8-a
	.file	"start.S"
    .extern main, relocate, apply_got, apply_rela, _stack_bot, _image_size
	.weak _DYNAMIC
	.hidden _DYNAMIC

	.global	_start

	.section ".init.text", "ax", @progbits

	.align	12
	.type	_start, %function

	// This resembles the Linux kernel Image header

	nop
	bl _start
	.quad	0				// Image load offset from start of RAM, little-endian
	.quad	_image_size		// Kernel size, little-endian
	.quad	0x0A			// Flags, little-endian: LE kernel(bit 0), 4K pages(bits 1-2), phys addr(bit 3)
	.quad	0				// Reserved
	.quad	0				// Reserved
	.quad	0				// Reserved
	.ascii	"ARM\x64"		// Signature

_start:
	// Set up the stack
    adrp    x20, _stack_bot
	sub		x20, x20, 64
    mov     sp, x20

    // Rust compiler produces globals for formatting calls,
    // need to relocate.

    adrp 	x0, _base
    mov 	x20, x0
    adrp 	x1, _got_start
    add 	x1, x1, :lo12:_got_start
    adrp 	x2, _got_end
    add 	x2, x2, :lo12:_got_end
	bl 		apply_got

    adrp 	x0, _base
    mov 	x20, x0
    adrp 	x1, _rela_start
    add 	x1, x1, :lo12:_rela_start
    adrp 	x2, _rela_end
    add 	x2, x2, :lo12:_rela_end
	bl 		apply_rela

    adrp 	x0, _base
    mov 	x20, x0
    adrp 	x1, _dynamic_start
    add 	x1, x1, :lo12:_dynamic_start
    adrp 	x2, _dynamic_end
    add 	x2, x2, :lo12:_dynamic_end
	bl 		apply_dynamic

	// Run the Rust main

	bl      main
	b       .
