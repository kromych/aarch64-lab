/* 
  objdump -i and objdump -f 
  print out all available options
*/

OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

SECTIONS
{
 /*   . = 0x8000;*/
    _IMAGE_START = .;

    _TEXT_BEGIN = .;
    .init.text : {
        *(.init.text*)
    }
    .text ALIGN(4K) : {
        *(.text*)
    }
    _TEXT_END = .;

    _RODATA_BEGIN = .;
    .rodata ALIGN(4K) : {
        *(.rodata*)
    }
    _RODATA_END = .;

    _RELRO_BEGIN = .;
    .data.rel.ro : {
        *(.data.rel.ro*)
    }
    _RELRO_END = .;

    _RELA_BEGIN = .;
    .rela.dyn : {
        *(.rela.dyn*)
    }
    _RELA_END = .;

    .dynamic : {
        *(.dynamic*)
    }

    _DATA_BEGIN = .;
    .data ALIGN(4K) : {
        *(.data*)
    }
    _DATA_END = .;

    _BSS_BEGIN = .;
    .bss ALIGN(4K) : {
        *(COMMON)
        *(.bss*)
        /* Allocate room for stack */
        .   =  ALIGN(4K);
        .   += 64K;
        _stack_top =  . - 16;        
    }
    _BSS_END = .;

    _IMAGE_END = .;
    _IMAGE_SIZE = _IMAGE_END - _IMAGE_START;

	/DISCARD/ : {
		*(.interp )
		*(.dynsym .dynstr .hash .gnu.hash)
	}
}
