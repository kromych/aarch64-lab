/* 
  objdump -i and objdump -f 
  print out all available options
*/

OUTPUT_FORMAT(elf64-aarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

PHDRS
{
    headers PT_PHDR PHDRS;
    text PT_LOAD FILEHDR PHDRS;
    rodata PT_LOAD;
    data PT_LOAD;
    dynamic PT_DYNAMIC;
}

SECTIONS
{
    . = SIZEOF_HEADERS;
    . = 4K;
    _IMAGE_START = .;

    .text : {
        _TEXT_BEGIN = .;
        *(.init.text*)
        *(.text*)
        _TEXT_END = .;
    } :text

    .init_array : ALIGN(4K) {
        _INIT_BEGIN = .;
        KEEP(*(SORT(.init_array.*)))
        KEEP(*(.init_array))
        _INIT_END = .;
    } :rodata

    .rodata : ALIGN(4K) {
        _RODATA_BEGIN = .;
        *(.rodata*)
        _RODATA_END = .;
    } :rodata

    .data.rel.ro : ALIGN(4K) {
        _RELRO_BEGIN = .;
        *(.data.rel.ro*)
        _RELRO_END = .;
    } :rodata

    .rela.dyn : ALIGN(4K) {
        _RELA_BEGIN = .;
        *(.rela.dyn*)
        _RELA_END = .;
    } :rodata

    .got.plt : ALIGN(4K) {
        _GOT_PLT_BEGIN = .;
        *(.got.plt*)
        _GOT_PLT_END = .;
    } :rodata

    .got : ALIGN(4K) {
        _GOT_BEGIN = .;
        *(.got*)
        _GOT_END = .;
    } :rodata

    .plt : ALIGN(4K) {
        _PLT_BEGIN = .;
        *(.plt*)
        _PLT_END = .;
    } :rodata

    .dynamic : ALIGN(4K) {
        _DYNAMIC = .;
        *(.dynamic*)
    } :rodata

    .data : ALIGN(4K) {
        _DATA_BEGIN = .;
        *(.data*)
        _DATA_END = .;
    } :data

    .bss : ALIGN(4K) {
        _BSS_BEGIN = .;
        *(.bss*)
        _BSS_END = .;
    } :data

    /* Allocate room for stack */
    .   =  ALIGN(4K);
    .   += 64K;
    _STACK_TOP = .;        

    _IMAGE_END = .;
    _IMAGE_SIZE = _IMAGE_END - _IMAGE_START;
}
