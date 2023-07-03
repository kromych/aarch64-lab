use crate::semihosting::Semihosting;
use core::fmt::Write;

const R_AARCH64_RELATIVE: u32 = 0x403;

#[repr(C, packed)]
#[derive(Debug)]
struct Elf64Dyn {
    tag: isize,
    val: usize,
}

#[repr(C, packed)]
struct Elf64Rela {
    offset: u64,
    info: u64,
    addend: u64,
}

fn rela_type(rela: &Elf64Rela) -> u32 {
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
            begin as *mut Elf64Rela,
            (end - begin) / core::mem::size_of::<Elf64Rela>(),
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

#[no_mangle]
fn apply_dynamic(_load_addr: u64, begin: usize, end: usize) {
    let dyna = unsafe {
        core::slice::from_raw_parts_mut(
            begin as *mut Elf64Dyn,
            (end - begin) / core::mem::size_of::<Elf64Dyn>(),
        )
    };
    let mut semi = Semihosting;
    for d in dyna {
        writeln!(semi, "{d:#x?}").ok();
    }
}
