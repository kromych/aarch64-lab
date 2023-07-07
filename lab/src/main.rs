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

mod image_data {
    extern "C" {
        fn _base();
        fn _end();
        fn _image_size();
    }

    pub fn base() -> usize {
        _base as usize
    }

    pub fn end() -> usize {
        _end as usize
    }

    pub fn size() -> usize {
        _image_size as usize
    }
}

mod reloc;

use aarch64::mmu;
use aarch64::mmu::PageTableSpace;
use aarch64::pl011;
use aarch64::pl011::PL011_BASE;
use aarch64::regs::access::Aarch64Register;
use aarch64::regs::*;
use aarch64::semihosting;

fn print_registers(out: &mut dyn core::fmt::Write) {
    let regs = [
        &mut CurrentEl::new() as &mut dyn Aarch64Register,
        &mut SystemControlEl1::new() as &mut dyn Aarch64Register,
        &mut VectorBaseEl1::new() as &mut dyn Aarch64Register,
        &mut MemoryAttributeIndirectionEl1::new() as &mut dyn Aarch64Register,
        &mut TranslationControlEl1::new() as &mut dyn Aarch64Register,
        &mut TranslationBase0El1::new() as &mut dyn Aarch64Register,
        &mut TranslationBase1El1::new() as &mut dyn Aarch64Register,
        &mut MmFeatures0El1::new() as &mut dyn Aarch64Register,
        &mut MmFeatures1El1::new() as &mut dyn Aarch64Register,
        &mut ExceptionLinkEl1::new() as &mut dyn Aarch64Register,
        &mut ExceptionSyndromeEl1::new() as &mut dyn Aarch64Register,
        &mut SavedProgramStateEl1::new() as &mut dyn Aarch64Register,
    ];

    for r in regs {
        r.read();

        let raw: u64 = r.bits();
        let name = r.name();
        writeln!(out, "{name}\t{raw:#016x?}: {r:x?}").ok();
    }
}

fn setup_mmu(out: &mut dyn core::fmt::Write) {
    let mut page_tables = PageTableSpace::new(
        page_table_space::page_tables_phys_start(),
        page_table_space::page_tables_area(),
    )
    .unwrap();
    writeln!(
        out,
        "Page tables are located at\t[{:#016x};{:#016x}]",
        page_table_space::page_tables_phys_start(),
        page_table_space::page_tables_phys_end()
    )
    .ok();

    let mut mair_el1 = MemoryAttributeIndirectionEl1::default();
    mair_el1.write();

    let page_size = mmu::PageSize::Small;
    page_tables
        .map_pages(
            image_data::base() as u64,
            mmu::VirtualAddress::from(image_data::base() as u64),
            core::cmp::max(image_data::size() / page_size as usize, 1),
            page_size,
            mair_el1
                .get_index(MemoryAttributeEl1::Normal_WriteBack)
                .expect("must be some WB memory available"),
        )
        .unwrap();

    page_tables
        .map_pages(
            PL011_BASE,
            mmu::VirtualAddress::from(PL011_BASE),
            1,
            mmu::PageSize::Small,
            mair_el1
                .get_index(MemoryAttributeEl1::Device_nGnRnE)
                .expect("must be some strongly ordered non-cacheable memory available"),
        )
        .unwrap();

    TranslationBase0El1::new()
        .with_asid(0)
        .with_baddr(page_table_space::page_tables_phys_start() as u64)
        .write();
    TranslationBase1El1::new().write();
    TranslationControlEl1::new()
        .with_t0sz(16)
        .with_irgn0(1)
        .with_orgn0(1)
        .with_sh0(3)
        .with_tg0(TranslationGranule0::_4KB)
        .with_epd1(1)
        .with_tg1(TranslationGranule1::_4KB)
        .with_ips(IntermPhysAddrSize::_48_bits_256TB)
        // .with_ha(1) // Should checked against the MMU feature reg #1
        // .with_hd(1) // Should checked against the MMU feature reg #1
        .write();

    writeln!(out, "Page tables use {:#x} bytes", page_tables.used_space()).ok();
    writeln!(out, "Enabling MMU").ok();

    let mut sctlr_el1 = SystemControlEl1::new();
    sctlr_el1.read();
    sctlr_el1.with_m(1).with_a(1).with_c(1).with_i(1).write();

    writeln!(out, "MMU enabled").ok();
}

const USE_SEMIHOSTING: bool = false;

#[no_mangle]
fn start() {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    let mut pl011: pl011::Pl011 = pl011::Pl011;
    let id = pl011.reset_and_init();

    let out = if USE_SEMIHOSTING {
        &mut semi as &mut dyn core::fmt::Write
    } else {
        &mut pl011 as &mut dyn core::fmt::Write
    };

    writeln!(out, "PL011 {id:#x}").ok();
    writeln!(
        out,
        "Image base {:#x}, end {:#x}, size {:#x}",
        image_data::base(),
        image_data::end(),
        image_data::size()
    )
    .ok();

    print_registers(out);
    setup_mmu(out);
    print_registers(out);

    writeln!(out, "Exiting").ok();
    if USE_SEMIHOSTING {
        semi.exit(0)
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    let mut pl011: pl011::Pl011 = pl011::Pl011;

    let out = if USE_SEMIHOSTING {
        &mut semi as &mut dyn core::fmt::Write
    } else {
        &mut pl011 as &mut dyn core::fmt::Write
    };

    if let Some(loc) = info.location() {
        writeln!(
            out,
            "\nPanic at {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column()
        )
        .ok();
    } else {
        writeln!(out, "\nPanic").ok();
    }

    if USE_SEMIHOSTING {
        semi.exit(!0)
    } else {
        loop {}
    }
}
