#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("start.S"));

mod page_table_space {
    extern "C" {
        fn _page_tables_start();
        fn _page_tables_end();
    }

    pub fn page_tables_phys_start() -> usize {
        _page_tables_start as usize
    }

    pub fn page_tables_phys_end() -> usize {
        _page_tables_end as usize
    }

    #[allow(dead_code)]
    pub fn page_tables_area() -> &'static mut [u8] {
        let s = page_tables_phys_start();
        let e = page_tables_phys_end();
        unsafe { core::slice::from_raw_parts_mut(s as *mut u8, e - s) }
    }
}

mod mmu;
mod pl011;
mod regs;
mod reloc;
mod semihosting;

use crate::regs::*;
use aarch64_cpu::registers::*;
use core::fmt::Write;
use mmu::PageTableSpace;
use tock_registers::interfaces::Readable;

#[no_mangle]
fn start() -> ! {
    let mut pl011: pl011::Pl011 = pl011::Pl011;
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    let id = pl011.reset_and_init();

    semi.write_char('H');
    pl011.write_str("ello ").ok();
    semi.write_hex(id);
    semi.write_char('\n');

    writeln!(semi, "Semihosting {id:#x}").ok();
    writeln!(pl011, "PL011 {id:#x}").ok();

    let current_el_raw = CurrentEL.get();
    let sctlr_el1_raw = SCTLR_EL1.get();
    let vbar_el1_raw = VBAR_EL1.get();
    let mair_el1_raw = MAIR_EL1.get();
    let tcr_el1_raw = TCR_EL1.get();
    let ttbr0_el1_raw = TTBR0_EL1.get();
    let ttbr1_el1_raw = TTBR1_EL1.get();
    let id_aa64mmfr0_el1_raw = ID_AA64MMFR0_EL1.get();
    let elr_el1_raw = ELR_EL1.get();
    let esr_el1_raw = ESR_EL1.get();
    let spsr_el1_raw = SPSR_EL1.get();

    let current_el = CurrentElVal::from(current_el_raw).el();
    let sctlr_el1 = SystemControlEl1Val::from(sctlr_el1_raw);
    let vbar_el1 = VectorBaseEl1Val::from(vbar_el1_raw);
    let mair_el1 = MemoryAttributeIndirectionEl1Val::from(mair_el1_raw);
    let tcr_el1 = TranslationControlEl1Val::from(tcr_el1_raw);
    let ttbr0_el1 = TranslationBaseEl1Val::from(ttbr0_el1_raw);
    let ttbr1_el1 = TranslationBaseEl1Val::from(ttbr1_el1_raw);
    let id_aa64mmfr0_el1 = MmuFeatures0El1Val::from(id_aa64mmfr0_el1_raw);

    writeln!(semi, "CurrentEL\t{current_el_raw:#016x?}: {current_el:?}").ok();
    writeln!(semi, "SCTLR_EL1\t{sctlr_el1_raw:#016x?}: {sctlr_el1:?}").ok();
    writeln!(
        semi,
        "Default SCTLR_EL1\t{:#016x?}: {:?}",
        u64::from(SystemControlEl1Val::default()),
        SystemControlEl1Val::default()
    )
    .ok();
    writeln!(semi, "VBAR_EL1\t{vbar_el1_raw:#016x?}: {vbar_el1:x?}").ok();
    writeln!(semi, "MAIR_EL1\t{mair_el1_raw:#016x?}: {mair_el1:x?}").ok();
    writeln!(semi, "TCR_EL1\t{tcr_el1_raw:#016x?}: {tcr_el1:?}").ok();
    writeln!(semi, "TTBR0_EL1\t{ttbr0_el1_raw:#016x?}: {ttbr0_el1:x?}").ok();
    writeln!(semi, "TTBR1_EL1\t{ttbr1_el1_raw:#016x?}: {ttbr1_el1:x?}").ok();
    writeln!(
        semi,
        "AA64MMFR0_EL1\t{id_aa64mmfr0_el1_raw:#016x?}: {id_aa64mmfr0_el1:?}"
    )
    .ok();
    writeln!(semi, "ELR_EL1\t{elr_el1_raw:#016x?}").ok();
    writeln!(semi, "ESR_EL1\t{esr_el1_raw:#016x?}").ok();
    writeln!(semi, "SPSR_EL1\t{spsr_el1_raw:#016x?}").ok();

    let mut page_tables = PageTableSpace::new(
        page_table_space::page_tables_phys_start(),
        page_table_space::page_tables_area(),
    )
    .unwrap();
    writeln!(
        semi,
        "Page tables are located at\t[{:#016x};{:#016x}]",
        page_table_space::page_tables_phys_start(),
        page_table_space::page_tables_phys_end()
    )
    .ok();

    page_tables
        .map_range(0, mmu::VirtualAddress::from(0), 0x4000000)
        .unwrap();

    semi.exit(0)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    if let Some(loc) = info.location() {
        writeln!(
            semi,
            "\nPanic at {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column()
        )
        .ok();
    } else {
        writeln!(semi, "\nPanic").ok();
    }

    semi.exit(!0)
}
