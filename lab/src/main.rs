#![no_std]
#![no_main]

const USE_SEMIHOSTING: bool = false;

// TODO: qemu virt-9.2 specific
const GICD_BASE: u64 = 0x08000000;
const GICD_SIZE: usize = 0x10000;
const GICR_BASE: u64 = 0x080a0000;
const GICR_SIZE: usize = 0xf60000;

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
        fn _payload_start();
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

    pub fn payload_start() -> usize {
        _payload_start as usize
    }
}

mod reloc;

use aarch64::gicv3::Gicv3;
use aarch64::mmu;
use aarch64::mmu::PageTableSpace;
use aarch64::pl011;
use aarch64::pl011::PL011_BASE;
use aarch64::register;
use aarch64::regs::access::Aarch64Register;
use aarch64::regs::*;
use aarch64::semihosting;
use core::sync::atomic::AtomicU32;

fn print_registers(out: &mut dyn core::fmt::Write) {
    let regs = [
        register!(MainIdEl1),
        register!(ProcessorFeatures0El1),
        register!(ProcessorFeatures1El1),
        register!(MmFeatures0El1),
        register!(MmFeatures1El1),
        register!(MmFeatures2El1),
        register!(MmFeatures3El1),
        register!(MmFeatures4El1),
        register!(CurrentEl),
        register!(SystemControlEl1),
        register!(VectorBaseEl1),
        register!(MemoryAttributeIndirectionEl1),
        register!(TranslationControlEl1),
        register!(TranslationBase0El1),
        register!(TranslationBase1El1),
        register!(ExceptionLinkEl1),
        register!(ExceptionSyndromeEl1),
        register!(SavedProgramStateEl1),
    ];

    for r in regs {
        r.load();

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
    mair_el1.store();

    page_tables
        .map_range(
            image_data::base() as u64,
            mmu::VirtualAddress::from(image_data::base() as u64),
            image_data::size() as u64,
            mair_el1
                .get_index(MemoryAttributeEl1::Normal_WriteBack)
                .expect("must be some WB memory available"),
        )
        .unwrap();

    let payload_size = 3 * 1024 * 1024;
    page_tables
        .map_range(
            image_data::payload_start() as u64,
            mmu::VirtualAddress::from(image_data::payload_start() as u64),
            payload_size,
            mair_el1
                .get_index(MemoryAttributeEl1::Normal_WriteBack)
                .expect("must be some WB memory available"),
        )
        .unwrap();

    page_tables
        .map_range(
            GICD_BASE,
            mmu::VirtualAddress::from(GICD_BASE),
            GICD_SIZE as u64,
            mair_el1
                .get_index(MemoryAttributeEl1::Device_nGnRnE)
                .expect("must be some device attrs available"),
        )
        .unwrap();

    page_tables
        .map_range(
            GICR_BASE,
            mmu::VirtualAddress::from(GICR_BASE),
            GICR_SIZE as u64,
            mair_el1
                .get_index(MemoryAttributeEl1::Device_nGnRnE)
                .expect("must be some device attrs available"),
        )
        .unwrap();

    writeln!(
        out,
        "running stride test at {:#x}",
        image_data::payload_start()
    )
    .ok();

    let dword_count = check_page_stride(
        image_data::payload_start() as u64,
        (image_data::payload_start() + payload_size as usize) as u64,
    );
    writeln!(out, "dword count: {dword_count:#x}").ok();

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
        .store();
    TranslationBase1El1::new().store();
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
        .store();

    writeln!(out, "Page tables use {:#x} bytes", page_tables.used_space()).ok();
    writeln!(
        out,
        "Page tables allocated for each level: {:?}",
        page_tables.lvl_stats()
    )
    .ok();
    writeln!(out, "Enabling MMU").ok();

    let mut sctlr_el1 = SystemControlEl1::new();
    sctlr_el1.load();
    sctlr_el1.with_m(1).with_a(1).with_c(1).with_i(1).store();

    writeln!(out, "MMU enabled").ok();

    writeln!(
        out,
        "running stride test at {:#x}",
        image_data::payload_start()
    )
    .ok();

    let dword_count = check_page_stride(
        image_data::payload_start() as u64,
        (image_data::payload_start() + payload_size as usize) as u64,
    );
    writeln!(out, "dword count: {dword_count:#x}").ok();
}

/// This function "generates" a function that adds 1 to
/// its first argument several timesand returns the result.
/// The two building blocks are these two instructions:
///
/// ```asm
///         add x0,x0,1 // 00 04 00 91
///         ret         // C0 03 5F D6
/// ```
fn check_page_stride(start: u64, end: u64) -> usize {
    let add_x0_x0_1 = 0x9100_0400_u32;
    let ret = 0xd65f_03c0_u32;

    if start >= end || (end - start) & 3 != 0 {
        panic!("expected a non-empty region of 32-bit words");
    }

    let code_space = unsafe {
        core::slice::from_raw_parts_mut(
            start as *mut u32,
            ((end - start) as usize) / core::mem::size_of::<u32>(),
        )
    };
    let code_space_size = code_space.len();
    code_space[..code_space_size - 1].fill(add_x0_x0_1);
    code_space[code_space_size - 1] = ret;

    let dword_counter: extern "C" fn(usize) -> usize =
        unsafe { core::mem::transmute(code_space.as_ptr()) };
    dword_counter(1)
}

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

    // Try exception handler
    // unsafe {
    //     core::arch::asm!(".byte 0xff, 0xff, 0xff, 0xff");
    // }
    // unsafe {
    //     let oops: *mut u64 = !0u64 as *mut u64;
    //     *oops = 0xdeadbeef;
    // }

    // TODO: Map the GICD and GICR regions to the virtual address space
    // if MMU is enabled

    writeln!(out, "Initialing GICv3").ok();

    let mut gic = Gicv3::new(
        unsafe { core::slice::from_raw_parts_mut(GICD_BASE as *mut AtomicU32, GICD_SIZE) },
        unsafe { core::slice::from_raw_parts_mut(GICR_BASE as *mut AtomicU32, GICR_SIZE) },
    );
    gic.init();

    writeln!(out, "Initialized GICv3").ok();

    const TEST_SGI: u32 = 10;

    gic.enable_interrupt(TEST_SGI);
    gic.enable_local_interrupts();

    // Enable interrupts at CPU level
    unsafe { core::arch::asm!("msr daifclr, #2") };

    gic.send_sgi(TEST_SGI, 1 << 0); // Target CPU 0

    unsafe { core::arch::asm!("1: wfi; b 1b") };

    writeln!(
        out,
        "Exiting, hit Ctrl+A X if semihosting is not compiled in"
    )
    .ok();
    if USE_SEMIHOSTING {
        semi.exit(0)
    } else {
        unsafe { core::arch::asm!("1: wfe; b 1b") };
    }
}

#[no_mangle]
unsafe extern "C" fn exception_handler(
    source: ExceptionSource,
    kind: ExceptionKind,
    exception_frame: *const ExceptionFrame,
) {
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    let mut pl011: pl011::Pl011 = pl011::Pl011;

    let out = if USE_SEMIHOSTING {
        &mut semi as &mut dyn core::fmt::Write
    } else {
        &mut pl011 as &mut dyn core::fmt::Write
    };

    writeln!(out, "**************************************************").ok();
    writeln!(out, "EXCEPTION source {source:?} kind {kind:?}").ok();

    let frame = unsafe { core::slice::from_raw_parts(exception_frame, 1) };
    writeln!(out, "Exception frame {frame:x?}").ok();

    // Get the interesting registers
    let regs = [
        register!(CurrentEl),
        register!(ExceptionLinkEl1),
        register!(ExceptionSyndromeEl1),
    ];
    for r in regs {
        r.load();

        let raw: u64 = r.bits();
        let name = r.name();
        writeln!(out, "{name}\t{raw:#016x?}: {r:x?}").ok();
    }
    writeln!(out, "**************************************************").ok();

    writeln!(
        out,
        "Exiting, hit Ctrl+A X if semihosting is not compiled in"
    )
    .ok();
    if USE_SEMIHOSTING {
        semi.exit(0)
    }

    unsafe { core::arch::asm!("b .") };
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
            "\nPANIC at {}:{}:{}",
            loc.file(),
            loc.line(),
            loc.column()
        )
        .ok();
    }

    let msg = info.message();
    writeln!(out, "PANIC: {}\n", msg).ok();

    if USE_SEMIHOSTING {
        semi.exit(!0)
    } else {
        loop {}
    }
}
