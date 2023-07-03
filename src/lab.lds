/* 
  objdump -i and objdump -f 
  print out all available options
*/

OUTPUT_FORMAT(elf64-aarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

_stack_size = 0x20000;

. = 0;

PHDRS
{
    hdr     PT_LOAD;
    text    PT_LOAD;
    rodata  PT_LOAD;
    data    PT_LOAD;
    dynamic PT_DYNAMIC;
}

SECTIONS {
    _base = .;

    _text_start = .;
    .init : ALIGN(0x4000) {
        *(.init)
        *(.init.*)
    } :text
    .text : ALIGN(0x4000) {
        *(.text)
        *(.text.*)
        . = ALIGN(8);
        *(.got.plt)
        . = ALIGN(0x4000);
    } :text
    _text_size = . - _text_start;
    .rodata : ALIGN(0x4000) {
        *(.rodata)
        *(.rodata.*)
        . = ALIGN(8);
    } :rodata
    .rela.dyn : {
        _rela_start = .;
        *(.rela)
        *(.rela.text)
        *(.rela.got)
        *(.rela.plt)
        *(.rela.bss)
        *(.rela.ifunc)
        *(.rela.text.*)
        *(.rela.data)
        *(.rela.data.*)
        *(.rela.rodata)
        *(.rela.rodata*)
        *(.rela.dyn)
        _rela_end = .;
        . = ALIGN(0x4000);
    } :rodata
    _rodata_end = .;
    _data_start = .;
    .data : ALIGN(0x4000) {
        *(.data)
        *(.data.*)
        . = ALIGN(8);
        _got_start = .;
        *(.got)
        _got_end = .;
        . = ALIGN(0x4000);
        _file_end = .;
    } :data
    .dynamic : ALIGN(0x4000){
        _dynamic_start = .;
        *(.dynamic)
        _dynamic_end = .;
    } :data :dynamic
    .bss : ALIGN(0x4000) {
        _bss_start = .;
        *(.bss)
        *(.bss.*)
        *(.dynbss)
        *(COMMON)
        _bss_end = .;
    } : data
    .stack : ALIGN(0x4000) {
        PROVIDE(_stack_top = .);
        . += _stack_size - 8;
        QUAD(0x544f424b43415453);
        PROVIDE(_stack_bot = .);
    } :data
    ASSERT(ALIGN(0x4000) == ., "Stack size is not aligned!")
    _data_size = . - _data_start;
    _end = .;
    _image_size = _end - _base;
    _payload_start = .;

    /DISCARD/ : {
        *(.discard)
        *(.discard.*)
        *(.interp .dynamic)
        *(.dynsym .dynstr .hash .gnu.hash)
        *(.eh_frame)
        *(.gnu.version*)
        *(.note*)
        *(.comment*)
    }

    .empty (NOLOAD) : {
        *(.plt) *(.plt.*) *(.iplt) *(.igot)
        *(.data.rel.ro)
    }
    ASSERT(SIZEOF(.empty) == 0, "Unexpected sections detected!")

    .got.plt (NOLOAD) : {
        *(.got.plt)
    }
    ASSERT(SIZEOF(.got.plt) == 0 || SIZEOF(.got.plt) == 0x18, "Unexpected GOT PLT detected!")
}
