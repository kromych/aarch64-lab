#![no_std]
#![no_main]

use core::arch::asm;

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

mod reloc;

use aarch64::mmu;
use aarch64::mmu::PageTableSpace;
//use aarch64::pl011;
use aarch64::regs::*;
use aarch64::semihosting;
use core::fmt::Write;

fn print_registers() {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;

    let current_el = CurrentEl::get();
    let sctlr_el1 = SystemControlEl1::get();
    let vbar_el1 = VectorBaseEl1::get();
    let mair_el1 = MemoryAttributeIndirectionEl1::get();
    let tcr_el1 = TranslationControlEl1::get();
    let ttbr0_el1 = TranslationBase0El1::get();
    let ttbr1_el1 = TranslationBase1El1::get();
    let id_aa64mmfr0_el1 = MmuFeatures0El1::get();
    let elr_el1 = aarch64::get_sys_reg!(ELR_EL1);
    let esr_el1 = aarch64::get_sys_reg!(ESR_EL1);
    let spsr_el1 = aarch64::get_sys_reg!(SPSR_EL1);

    let current_el_raw: u64 = current_el.into();
    let sctlr_el1_raw: u64 = sctlr_el1.into();
    let vbar_el1_raw: u64 = vbar_el1.into();
    let mair_el1_raw: u64 = mair_el1.into();
    let tcr_el1_raw: u64 = tcr_el1.into();
    let ttbr0_el1_raw: u64 = ttbr0_el1.into();
    let ttbr1_el1_raw: u64 = ttbr1_el1.into();
    let id_aa64mmfr0_el1_raw: u64 = id_aa64mmfr0_el1.into();
    let elr_el1_raw: u64 = elr_el1;
    let esr_el1_raw: u64 = esr_el1;
    let spsr_el1_raw: u64 = spsr_el1;

    writeln!(semi, "CurrentEL\t{current_el_raw:#016x?}: {current_el:?}").ok();
    writeln!(semi, "SCTLR_EL1\t{sctlr_el1_raw:#016x?}: {sctlr_el1:?}").ok();
    writeln!(
        semi,
        "Default SCTLR_EL1\t{:#016x?}: {:?}",
        u64::from(SystemControlEl1::default()),
        SystemControlEl1::default()
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
}

fn setup_mmu() {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;

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
        .map_pages(
            0x4000_0000,
            mmu::VirtualAddress::from(0x4000_0000),
            0x200_0000,
            mmu::PageSize::Small,
        )
        .unwrap();

    MemoryAttributeIndirectionEl1::default().set();
    TranslationBase0El1::new()
        .with_asid(0)
        .with_baddr(page_table_space::page_tables_phys_start() as u64)
        .set();
    TranslationBase1El1::new().set();
    TranslationControlEl1::new()
        .with_t0sz(16)
        .with_irgn0(1)
        .with_orgn0(1)
        .with_sh0(3)
        .with_tg0(TranslationGranule0::_4KB)
        .with_epd1(1)
        .with_tg1(TranslationGranule1::_4KB)
        .with_ips(IntermPhysAddrSize::_48_bits_256TB)
        .set();

    writeln!(semi, "Enabling MMU").ok();

    let sctlr_el1 = SystemControlEl1::get();
    sctlr_el1.with_m(1).set();

    writeln!(semi, "MMU enabled").ok();
}

fn use_pl011() {
    // let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    // let mut pl011: pl011::Pl011 = pl011::Pl011;
    // let id = pl011.reset_and_init();

    // semi.write_char('H');
    // pl011.write_str("ello ").ok();
    // semi.write_hex(id);
    // semi.write_char('\n');

    // writeln!(semi, "Semihosting {id:#x}").ok();
    // writeln!(pl011, "PL011 {id:#x}").ok();
}

#[no_mangle]
fn start() -> ! {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;

    use_pl011();
    print_registers();
    setup_mmu();
    print_registers();

    writeln!(semi, "Exiting").ok();
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
