use crate::semihosting::write_char;
use crate::semihosting::write_hex;

extern "C" {
    fn _IMAGE_START();
    fn _RELA_BEGIN();
    fn _RELA_END();
    fn _GOT_BEGIN();
    fn _GOT_END();
}

const R_AARCH64_RELATIVE: u32 = 1027;

#[repr(C, packed)]
struct elf64_rela {
    offset: usize,
    info: u64,
    addend: usize,
}

fn rela_type(rela: &elf64_rela) -> u32 {
    rela.info as u32
}

fn relocate_got(load_addr: usize, begin: usize, end: usize) {
    let got_array = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut usize,
            (end - begin) / core::mem::size_of::<usize>(),
        )
    };
    for var in got_array {
        write_char('-');
        write_hex(*var as u64);
        write_char('\n');

        *var = var.wrapping_add(load_addr);
    }
}

fn relocate_rela(load_addr: usize, begin: usize, end: usize) {
    let rela = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut elf64_rela,
            (end - begin) / core::mem::size_of::<elf64_rela>(),
        )
    };
    for rel in rela {
        while rela_type(rel) != R_AARCH64_RELATIVE {}

        unsafe {
            *((rel.offset + load_addr) as *mut usize) = rel.addend + load_addr;
        }
    }
}

#[no_mangle]
fn relocate(load_addr: usize) {
    let image_start = _IMAGE_START as usize;
    write_char(' ');
    write_hex(image_start as u64);

    let got_begin = (_GOT_BEGIN as usize)
        .wrapping_add(load_addr)
        .wrapping_sub(image_start);
    let got_end = (_GOT_END as usize)
        .wrapping_add(load_addr)
        .wrapping_sub(image_start);

    write_char(' ');
    write_hex(got_begin as u64);

    write_char(' ');
    write_hex(got_end as u64);

    write_char(' ');
    write_hex(load_addr as u64);
    write_char('\n');

    relocate_got(load_addr, got_begin, got_end);

    let rela_begin = (_RELA_BEGIN as usize)
        .wrapping_add(load_addr)
        .wrapping_sub(image_start);
    let rela_end = (_RELA_END as usize)
        .wrapping_add(load_addr)
        .wrapping_sub(image_start);
    relocate_rela(load_addr, rela_begin, rela_end);
}
