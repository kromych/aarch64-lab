extern "C" {
    fn _base();
    fn _rela_start();
    fn _rela_end();
    fn _got_start();
    fn _got_end();
}

const R_AARCH64_RELATIVE: u32 = 0x403;

#[repr(C, packed)]
struct elf64_rela {
    offset: u64,
    info: u64,
    addend: u64,
}

fn rela_type(rela: &elf64_rela) -> u32 {
    rela.info as u32
}

#[no_mangle]
fn apply_got(load_addr: u64, begin: usize, end: usize) {
    let got = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut u64,
            (end - begin) / core::mem::size_of::<u64>(),
        )
    };
    for var in got {
        *var = var.wrapping_add(load_addr);
    }
}

#[no_mangle]
fn apply_rela(load_addr: u64, begin: usize, end: usize) {
    let rela = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut elf64_rela,
            (end - begin) / core::mem::size_of::<elf64_rela>(),
        )
    };
    for rel in rela {
        if rela_type(rel) != R_AARCH64_RELATIVE {
            panic!("rela")
        }

        unsafe {
            *((rel.offset + load_addr) as *mut u64) = rel.addend + load_addr;
        }
    }
}
