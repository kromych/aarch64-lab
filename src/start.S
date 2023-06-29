	.arch armv8-a
	.file	"start.S"
    .extern main, relocate, _IMAGE_SIZE, _stack_top
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
    adr     x20, _stack_top
    mov     sp, x20
	bl 		relocate
	bl      main
	b       .
	.size	_start, .-_start
