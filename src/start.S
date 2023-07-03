	.arch armv8-a
	.file	"start.S"
    .extern main, relocate, _IMAGE_SIZE, _STACK_TOP
	.global	_start

	.section ".init.text", "ax", @progbits

	.align	12
	.type	_start, %function
	nop
	bl _start
	.quad	0			// Image load offset from start of RAM, little-endian
	.quad	_IMAGE_SIZE	// Kernel size, little-endian
	.quad	0x0A		// Flags, little-endian: LE kernel(bit 0), 4K pages(bits 1-2), phys addr(bit 3)
	.quad	0			// Reserved
	.quad	0			// Reserved
	.quad	0			// Reserved
	.ascii	"ARM\x64"	// Signature

_start:
	// Set up the stack
    adrp    x20, _STACK_TOP
	sub		x20, x20, 64
    mov     sp, x20

    adrp	x0, .
	bl 		relocate

	bl      main
	b       .
	.size	_start, .-_start
