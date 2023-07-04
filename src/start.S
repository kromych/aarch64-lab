	.arch armv8-a
	.file	"start.S"
    .extern main, _stack_bot, _image_size, relocate
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
