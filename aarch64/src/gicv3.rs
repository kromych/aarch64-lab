//! Lean and mean GICv3 interface.
//!
//! GICv3 has two main components:
//! 1. GICD (distributor) - the central hub for all interrupts
//! 2. GICR (re-distributors) - per-CPU interrupt management
//!
//! GICD functions, most notably:
//! - routes interrupts to correct CPU,
//! - stores global interrupt state (enabled/pending),
//! - handles interrupt prioritization,
//! - broadcasts SGIs (Software Generated Interrupts)
//!
//! The distributor has the size of 64KiB.
//!
//! Crucial GICR functions (per CPU):
//! - manages CPU-private interrupts (SGIs 0-15, PPIs 16-31),
//! - handles interrupt signaling to individual cores,
//! - provides wakeup control for power management
//!
//! Each redistributor defines two 64KiB frames in the physical address map:
//! - RD_base for controlling the overall behavior of the Redistributor, for
//!   controlling LPIs, and for generating LPIs in a system that does not
//!   include at least one ITS,
//! - SGI_base for controlling and generating PPIs and SGIs.
//!
//! For overview, refer to [GICv3 and GICv4 Software Overview](https://developer.arm.com/documentation/dai0492/b/)
//! See [GIC architecture version 3 and version 4](https://developer.arm.com/documentation/ihi0069/latest/)
//! for more details. That document is refernced below unless stated otherwise.
//!
//! NOTE: Make sure to read GIC by 32-bits aligned!

use bitfield_struct::bitfield;
use core::mem::offset_of;
use static_assertions;
use static_assertions::const_assert;

pub const GICD_SIZE: usize = 0x10000;

// GIC registers, "12.9 The GIC Distributor register descriptions"

/// Disributor control
#[bitfield(u32)]
pub struct GicdCtrl {
    #[bits(1)]
    pub enable_grp0: u32,
    #[bits(1)]
    pub enable_grp1_ns: u32,
    #[bits(1)]
    pub enable_grp1_s: u32,
    #[bits(1)]
    pub _res0: u32,
    #[bits(1)]
    pub are_s: u32,
    #[bits(1)]
    pub are_ns: u32,
    #[bits(1)]
    pub disable_secure: u32,
    #[bits(1)]
    pub e1_nwf: u32,
    #[bits(23)]
    pub _res1: u32,
    #[bits(1)]
    pub reg_write_pending: u32,
}

impl GicdCtrl {
    fn reset(&mut self) {
        self.0 = 0;
    }

    fn wait_pending_write(&self) {
        let mut count = 0x100_0000_i32;
        while self.reg_write_pending() != 0 {
            count -= 1;
            if count.is_negative() {
                panic!("arm_gicv3: rwp timeout");
            }
        }
    }
}

/// Identification register
#[bitfield(u32)]
pub struct GicdIidr {
    #[bits(12)]
    pub implementer: u32,
    #[bits(4)]
    pub revision: u32,
    #[bits(4)]
    pub variant: u32,
    #[bits(4)]
    pub _res0: u32,
    #[bits(8)]
    pub product_id: u32,
}

/// Distributor information
#[bitfield(u32)]
pub struct GicdTyper {
    #[bits(5)]
    pub it_lines: u32,
    #[bits(3)]
    pub cpu_number: u32,
    #[bits(1)]
    pub espi: u32,
    #[bits(1)]
    pub nmi: u32,
    #[bits(1)]
    pub security_extn: u32,
    #[bits(5)]
    pub lpi_lines: u32,
    #[bits(1)]
    pub mbis: u32,
    #[bits(1)]
    pub lpis: u32,
    #[bits(1)]
    pub dvis: u32,
    #[bits(5)]
    pub id_bits: u32,
    #[bits(1)]
    pub a3v: u32,
    #[bits(1)]
    pub no1n: u32,
    #[bits(1)]
    pub rss: u32,
    #[bits(5)]
    pub espi_range: u32,
}

#[bitfield(u32)]
/// Peripheral ID2 Register
pub struct GicPidr2 {
    #[bits(4)]
    pub _impl_def0: u32,
    #[bits(4)]
    pub gic_version: u32,
    #[bits(24)]
    pub _impl_def1: u32,
}

/// GIC Distributor register map
///
/// This struct represents the memory-mapped registers of the ARM Generic Interrupt Controller (GIC) Distributor.
/// All registers are 32-bit wide and aligned to 4 bytes.
///
/// See "12.8 The GIC Distributor register map".
#[repr(C, align(0x10000))]
pub struct GicDistributor {
    /// 0x0000 - Distributor Control Register
    ///
    /// Controls overall operation of the Distributor. The reset value is implementation defined.
    pub ctlr: GicdCtrl, // GICD_CTLR

    /// 0x0004 - Interrupt Controller Type Register
    ///
    /// Provides information about the configuration of the GIC. Read-only, implementation defined.
    pub typer: GicdTyper, // GICD_TYPER

    /// 0x0008 - Distributor Implementer Identification Register
    ///
    /// Identifies the implementer of the GIC. Read-only, implementation defined.
    pub iidr: GicdIidr, // GICD_IIDR

    /// 0x000C - Interrupt Controller Type Register 2
    ///
    /// Additional type information about the GIC. Read-only, implementation defined.
    pub typer2: u32, // GICD_TYPER2

    /// 0x0010 - Error Reporting Status Register (optional)
    ///
    /// Reports error conditions in the Distributor. Reset value: 0x00000000
    pub statusr: u32, // GICD_STATUSR

    // 0x0014-0x001C - Reserved
    _reserved_0014: [u32; 3],

    // 0x0020-0x003C - IMPLEMENTATION DEFINED registers
    _impl_defined0: [u32; 8],

    /// 0x0040 - Set SPI Register (Non-secure)
    ///
    /// Writing to this register sets the corresponding SPI interrupt pending state in the non-secure state.
    pub setspi_nsr: u32, // GICD_SETSPI_NSR

    // 0x0044 - Reserved
    _reserved_0044: u32,

    /// 0x0048 - Clear SPI Register (Non-secure)
    ///
    /// Writing to this register clears the corresponding SPI interrupt pending state in the non-secure state.
    pub clrspi_nsr: u32, // GICD_CLRSPI_NSR

    // 0x004C - Reserved
    _reserved_004c: u32,

    /// 0x0050 - Set SPI Register (Secure)
    ///
    /// Writing to this register sets the corresponding SPI interrupt pending state in the secure state.
    pub setspi_sr: u32, // GICD_SETSPI_SR

    // 0x0054 - Reserved
    _reserved_0054: u32,

    /// 0x0058 - Clear SPI Register (Secure)
    ///
    /// Writing to this register clears the corresponding SPI interrupt pending state in the secure state.
    pub clrspi_sr: u32, // GICD_CLRSPI_SR

    // 0x005C-0x007C - Reserved
    _reserved_005c: [u32; 9],

    /// 0x0080-0x00FC - Interrupt Group Registers
    ///
    /// Configures interrupts as Group 0 or Group 1. Reset value: implementation defined.
    pub igroupr: [u32; 32], // GICD_IGROUPR<n>

    /// 0x0100-0x017C - Interrupt Set-Enable Registers
    ///
    /// Enables forwarding of interrupts to CPU interfaces. Reset value: implementation defined.
    pub isenabler: [u32; 32], // GICD_ISENABLER<n>

    /// 0x0180-0x01FC - Interrupt Clear-Enable Registers
    ///
    /// Disables forwarding of interrupts to CPU interfaces. Reset value: implementation defined.
    pub icenabler: [u32; 32], // GICD_ICENABLER<n>

    /// 0x0200-0x027C - Interrupt Set-Pending Registers
    ///
    /// Sets the pending state of interrupts. Reset value: 0x00000000
    pub ispendr: [u32; 32], // GICD_ISPENDR<n>

    /// 0x0280-0x02FC - Interrupt Clear-Pending Registers
    ///
    /// Clears the pending state of interrupts. Reset value: 0x00000000
    pub icpendr: [u32; 32], // GICD_ICPENDR<n>

    /// 0x0300-0x037C - Interrupt Set-Active Registers
    ///
    /// Sets the active state of interrupts. Reset value: 0x00000000
    pub isactiver: [u32; 32], // GICD_ISACTIVER<n>

    /// 0x0380-0x03FC - Interrupt Clear-Active Registers
    ///
    /// Clears the active state of interrupts. Reset value: 0x00000000
    pub icactiver: [u32; 32], // GICD_ICACTIVER<n>

    /// 0x0400-0x07F8 - Interrupt Priority Registers
    ///
    /// Configures the priority of each interrupt. Reset value: 0x00000000
    pub ipriorityr: [u32; 256], // GICD_IPRIORITYR<n>

    /// 0x0800-0x081C - Interrupt Processor Targets Registers
    ///
    /// Configures which CPUs receive each interrupt. Read-only, implementation defined.
    /// RES0 when affinity routing is enabled.
    pub itargetsr: [u32; 8], // GICD_ITARGETSR<n>

    // 0x0820-0x0BFC - Reserved
    _reserved_0820: [u32; 248],

    /// 0x0C00-0x0CFC - Interrupt Configuration Registers
    ///
    /// Configures interrupts as level-sensitive or edge-triggered. Reset value: implementation defined.
    pub icfgr: [u32; 64], // GICD_ICFGR<n>

    /// 0x0D00-0x0D7C - Interrupt Group Modifier Registers
    ///
    /// Modifies interrupt group behavior. Reset value: 0x00000000
    pub igrpmodr: [u32; 32], // GICD_IGRPMODR<n>

    // 0x0D80-0x0DFC - Reserved
    _reserved_0d80: [u32; 32],

    /// 0x0E00-0x0EFC - Non-secure Access Control Registers
    ///
    /// Controls non-secure access to secure interrupts. Reset value: 0x00000000
    pub nsacr: [u32; 64], // GICD_NSACR<n>

    /// 0x0F00 - Software Generated Interrupt Register
    ///
    /// Generates software interrupts. RES0 when affinity routing is enabled.
    pub sgir: u32, // GICD_SGIR

    // 0x0F04-0x0F0C - Reserved
    _reserved_0f04: [u32; 3],

    /// 0x0F10-0x0F1C - SGI Clear-Pending Registers
    ///
    /// Clears pending state of SGIs. Reset value: 0x00000000
    /// RES0 when affinity routing is enabled.
    pub cpendsgir: [u32; 4], // GICD_CPENDSGIR<n>

    /// 0x0F20-0x0F2C - SGI Set-Pending Registers
    ///
    /// Sets pending state of SGIs. Reset value: 0x00000000
    /// RES0 when affinity routing is enabled.
    pub spendsgir: [u32; 4], // GICD_SPENDSGIR<n>

    // 0x0F30-0x0F7C - Reserved
    _reserved_0f30: [u32; 20],

    /// 0x0F80-0x0FFC - Non-maskable Interrupt Registers
    ///
    /// Controls non-maskable interrupts. Reset value: 0x00000000
    pub inmir: [u32; 32], // GICD_INMIR<n>

    // Extended SPI registers (0x1000 onwards)
    /// 0x1000-0x107C - Interrupt Group Registers for extended SPI range
    ///
    /// Configures extended SPIs as Group 0 or Group 1. Reset value: 0x00000000
    pub igroupr_e: [u32; 32], // GICD_IGROUPR<n>E

    // 0x1080-0x11FC - Reserved
    _reserved_1080: [u32; 96],

    /// 0x1200-0x127C - Interrupt Set-Enable for extended SPI range
    ///
    /// Enables forwarding of extended SPI interrupts. Reset value: implementation defined.
    pub isenabler_e: [u32; 32], // GICD_ISENABLER<n>E

    // 0x1280-0x13FC - Reserved
    _reserved_1280: [u32; 96],

    /// 0x1400-0x147C - Interrupt Clear-Enable for extended SPI range
    ///
    /// Disables forwarding of extended SPI interrupts. Reset value: implementation defined.
    pub icenabler_e: [u32; 32], // GICD_ICENABLER<n>E

    // 0x1480-0x15FC - Reserved
    _reserved_14800: [u32; 96],

    /// 0x1600-0x167C - Interrupt Set-Pend for extended SPI range
    ///
    /// Sets the pending state of extended SPI interrupts. Reset value: 0x00000000
    pub ispendr_e: [u32; 32], // GICD_ISPENDR<n>E

    // 0x1680-0x17FC - Reserved
    _reserved_16801: [u32; 96],

    /// 0x1800-0x187C - Interrupt Clear-Pend for extended SPI range
    ///
    /// Clears the pending state of extended SPI interrupts. Reset value: 0x00000000
    pub icpendr_e: [u32; 32], // GICD_ICPENDR<n>E

    // 0x1880-0x19FC - Reserved
    _reserved_18802: [u32; 96],

    /// 0x1A00-0x1A7C - Interrupt Set-Active for extended SPI range
    ///
    /// Sets the active state of extended SPI interrupts. Reset value: 0x00000000
    pub isactiver_e: [u32; 32], // GICD_ISACTIVER<n>E

    // 0x1A80-0x1BFC - Reserved
    _reserved_1a803: [u32; 96],

    /// 0x1C00-0x1C7C - Interrupt Clear-Active for extended SPI range
    ///
    /// Clears the active state of extended SPI interrupts. Reset value: 0x00000000
    pub icactiver_e: [u32; 32], // GICD_ICACTIVER<n>E

    // 0x1C80-0x1FFC - Reserved
    _reserved_1c804: [u32; 224],

    /// 0x2000-0x23FC - Interrupt Priority for extended SPI range
    ///
    /// Configures the priority of extended SPI interrupts. Reset value: 0x00000000
    pub ipriorityr_e: [u32; 256], // GICD_IPRIORITYR<n>E

    // 0x2400-0x2FFC - Reserved
    _reserved_24005: [u32; 768],

    /// 0x3000-0x30FC - Extended SPI Configuration Register
    ///
    /// Configures extended SPI interrupts as level-sensitive or edge-triggered. Reset value: implementation defined.
    pub icfgr_e: [u32; 64], // GICD_ICFGR<n>E

    // 0x3100-0x33FC - Reserved
    _reserved_31006: [u32; 192],

    /// 0x3400-0x347C - Interrupt Group Modifier for extended SPI range
    ///
    /// Modifies extended SPI interrupt group behavior. Reset value: 0x00000000
    pub igrpmodr_e: [u32; 32], // GICD_IGRPMODR<n>E

    // 0x3480-0x35FC - Reserved
    _reserved_34807: [u32; 96],

    /// 0x3600-0x36FC - Non-secure Access Control Registers for extended SPI range
    ///
    /// Controls non-secure access to secure extended SPI interrupts. Reset value: 0x00000000
    pub nsacr_e: [u32; 64], // GICD_NSACR<n>E

    // 0x3700-0x3AFC - Reserved
    _reserved_37008: [u32; 256],

    /// 0x3B00-0x3B7C - Non-maskable Interrupt Registers for Extended SPIs
    ///
    /// Controls non-maskable extended SPI interrupts. Reset value: 0x00000000
    pub inmir_e: [u32; 32], // GICD_INMIR<n>Eg

    // 0x3B80-0x60FC - Reserved
    _reserved_3b809: [u32; 2400],

    /// 0x6100-0x7FD8 - Interrupt Routing Registers
    ///
    /// Configures interrupt routing for affinity-based systems.
    pub irouter: [u32; 1984], // GICD_IROUTER<n>

    /// 0x8000-0x9FFC - Interrupt Routing Registers for extended SPI range
    ///
    /// Configures interrupt routing for extended SPI interrupts in affinity-based systems.
    pub irouter_e: [u32; 2048], // GICD_IROUTER<n>E

    // 0xA000-0xBFFC - Reserved
    _reserved_a0000: [u32; 2048],
    // 0xC000-0xFFCC - IMPLEMENTATION DEFINED registers
    _impl_defined1: [u32; 4084],
    // 0xFFD0-0xFFE4 - IMPLEMENTATION DEFINED registers
    _impl_defined_id0: [u32; 6],

    /// Distributor Peripheral ID2 Register
    pub pidr2: GicPidr2,

    // 0xFFEC-0xFFFC - IMPLEMENTATION DEFINED registers
    _impl_defined_id1: [u32; 5],
}

pub const GICD_CTLR_OFFSET: usize = 0x0000;
pub const GICD_TYPER_OFFSET: usize = 0x0004;
pub const GICD_IIDR_OFFSET: usize = 0x0008;
pub const GICD_TYPER2_OFFSET: usize = 0x000C;
pub const GICD_STATUSR_OFFSET: usize = 0x0010;
pub const GICD_SETSPI_NSR_OFFSET: usize = 0x0040;
pub const GICD_CLRSPI_NSR_OFFSET: usize = 0x0048;
pub const GICD_SETSPI_SR_OFFSET: usize = 0x0050;
pub const GICD_CLRSPI_SR_OFFSET: usize = 0x0058;
pub const GICD_IGROUPR_OFFSET: usize = 0x0080;
pub const GICD_ISENABLER_OFFSET: usize = 0x0100;
pub const GICD_ICENABLER_OFFSET: usize = 0x0180;
pub const GICD_ISPENDR_OFFSET: usize = 0x0200;
pub const GICD_ICPENDR_OFFSET: usize = 0x0280;
pub const GICD_ISACTIVER_OFFSET: usize = 0x0300;
pub const GICD_ICACTIVER_OFFSET: usize = 0x0380;
pub const GICD_IPRIORITYR_OFFSET: usize = 0x0400;
pub const GICD_ITARGETSR_OFFSET: usize = 0x0800;
pub const GICD_ICFGR_OFFSET: usize = 0x0C00;
pub const GICD_IGRPMODR_OFFSET: usize = 0x0D00;
pub const GICD_NSACR_OFFSET: usize = 0x0E00;
pub const GICD_SGIR_OFFSET: usize = 0x0F00;
pub const GICD_CPENDSGIR_OFFSET: usize = 0x0F10;
pub const GICD_SPENDSGIR_OFFSET: usize = 0x0F20;
pub const GICD_INMIR_OFFSET: usize = 0x0F80;
pub const GICD_IGROUPR_E_OFFSET: usize = 0x1000;
pub const GICD_ISENABLER_E_OFFSET: usize = 0x1200;
pub const GICD_ICENABLER_E_OFFSET: usize = 0x1400;
pub const GICD_ISPENDR_E_OFFSET: usize = 0x1600;
pub const GICD_ICPENDR_E_OFFSET: usize = 0x1800;
pub const GICD_ISACTIVER_E_OFFSET: usize = 0x1A00;
pub const GICD_ICACTIVER_E_OFFSET: usize = 0x1C00;
pub const GICD_IPRIORITYR_E_OFFSET: usize = 0x2000;
pub const GICD_ICFGR_E_OFFSET: usize = 0x3000;
pub const GICD_IGRPMODR_E_OFFSET: usize = 0x3400;
pub const GICD_NSACR_E_OFFSET: usize = 0x3600;
pub const GICD_INMIR_E_OFFSET: usize = 0x3B00;
pub const GICD_IROUTER_OFFSET: usize = 0x6100;
pub const GICD_IROUTER_E_OFFSET: usize = 0x8000;
pub const GICD_PIDR2_OFFSET: usize = 0xFFE8;

const_assert!(size_of::<GicDistributor>() == GICD_SIZE);

const_assert!(offset_of!(GicDistributor, ctlr) == GICD_CTLR_OFFSET);
const_assert!(offset_of!(GicDistributor, typer) == GICD_TYPER_OFFSET);
const_assert!(offset_of!(GicDistributor, iidr) == GICD_IIDR_OFFSET);
const_assert!(offset_of!(GicDistributor, typer2) == GICD_TYPER2_OFFSET);
const_assert!(offset_of!(GicDistributor, statusr) == GICD_STATUSR_OFFSET);
const_assert!(offset_of!(GicDistributor, setspi_nsr) == GICD_SETSPI_NSR_OFFSET);
const_assert!(offset_of!(GicDistributor, clrspi_nsr) == GICD_CLRSPI_NSR_OFFSET);
const_assert!(offset_of!(GicDistributor, setspi_sr) == GICD_SETSPI_SR_OFFSET);
const_assert!(offset_of!(GicDistributor, clrspi_sr) == GICD_CLRSPI_SR_OFFSET);
const_assert!(offset_of!(GicDistributor, igroupr) == GICD_IGROUPR_OFFSET);
const_assert!(offset_of!(GicDistributor, isenabler) == GICD_ISENABLER_OFFSET);
const_assert!(offset_of!(GicDistributor, icenabler) == GICD_ICENABLER_OFFSET);
const_assert!(offset_of!(GicDistributor, ispendr) == GICD_ISPENDR_OFFSET);
const_assert!(offset_of!(GicDistributor, icpendr) == GICD_ICPENDR_OFFSET);
const_assert!(offset_of!(GicDistributor, isactiver) == GICD_ISACTIVER_OFFSET);
const_assert!(offset_of!(GicDistributor, icactiver) == GICD_ICACTIVER_OFFSET);
const_assert!(offset_of!(GicDistributor, ipriorityr) == GICD_IPRIORITYR_OFFSET);
const_assert!(offset_of!(GicDistributor, itargetsr) == GICD_ITARGETSR_OFFSET);
const_assert!(offset_of!(GicDistributor, icfgr) == GICD_ICFGR_OFFSET);
const_assert!(offset_of!(GicDistributor, igrpmodr) == GICD_IGRPMODR_OFFSET);
const_assert!(offset_of!(GicDistributor, nsacr) == GICD_NSACR_OFFSET);
const_assert!(offset_of!(GicDistributor, sgir) == GICD_SGIR_OFFSET);
const_assert!(offset_of!(GicDistributor, cpendsgir) == GICD_CPENDSGIR_OFFSET);
const_assert!(offset_of!(GicDistributor, spendsgir) == GICD_SPENDSGIR_OFFSET);
const_assert!(offset_of!(GicDistributor, inmir) == GICD_INMIR_OFFSET);
const_assert!(offset_of!(GicDistributor, igroupr_e) == GICD_IGROUPR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, isenabler_e) == GICD_ISENABLER_E_OFFSET);
const_assert!(offset_of!(GicDistributor, icenabler_e) == GICD_ICENABLER_E_OFFSET);
const_assert!(offset_of!(GicDistributor, ispendr_e) == GICD_ISPENDR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, icpendr_e) == GICD_ICPENDR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, isactiver_e) == GICD_ISACTIVER_E_OFFSET);
const_assert!(offset_of!(GicDistributor, icactiver_e) == GICD_ICACTIVER_E_OFFSET);
const_assert!(offset_of!(GicDistributor, ipriorityr_e) == GICD_IPRIORITYR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, icfgr_e) == GICD_ICFGR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, igrpmodr_e) == GICD_IGRPMODR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, nsacr_e) == GICD_NSACR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, inmir_e) == GICD_INMIR_E_OFFSET);
const_assert!(offset_of!(GicDistributor, irouter) == GICD_IROUTER_OFFSET);
const_assert!(offset_of!(GicDistributor, irouter_e) == GICD_IROUTER_E_OFFSET);
const_assert!(offset_of!(GicDistributor, pidr2) == GICD_PIDR2_OFFSET);

pub const GICR_SIZE: usize = 0x20000;
pub const GICR_FRAME_SIZE: usize = 0x10000;

/// GIC physical LPI Redistributor register map
///
/// This struct represents the memory-mapped registers of the ARM GIC Redistributor
/// for physical LPIs. All registers are 32-bit wide and aligned to 4 bytes.
#[repr(C, align(4))]
pub struct GicLpiRedistributor {
    /// 0x0000 - Redistributor Control Register
    pub ctlr: u32, // GICR_CTLR

    /// 0x0004 - Implementer Identification Register
    pub iidr: u32, // GICR_IIDR

    /// 0x0008 - Redistributor Type Register
    pub typer: u32, // GICR_TYPER

    // 0x000C - Reserved
    _reserved0: u32,

    /// 0x0010 - Error Reporting Status Register (optional)
    pub statusr: u32, // GICR_STATUSR

    /// 0x0014 - Redistributor Wake Register
    pub waker: u32, // GICR_WAKER

    /// 0x0018 - Report maximum PARTID and PMG Register
    pub mpamidr: u32, // GICR_MPAMIDR

    /// 0x001C - Set PARTID and PMG Register
    pub partidr: u32, // GICR_PARTIDR

    // 0x0020-0x003C - IMPLEMENTATION DEFINED registers
    _impl_defined0: [u32; 8],

    /// 0x0040 - Set LPI Pending Register
    pub setlpir: u32, // GICR_SETLPIR

    // 0x0044 - Reserved
    _reserved1: u32,

    /// 0x0048 - Clear LPI Pending Register
    pub clrlpir: u32, // GICR_CLRLPIR

    // 0x004C - Reserved
    _reserved2: u32,

    // 0x0050-0x006C - Reserved
    _reserved3: [u32; 8],

    /// 0x0070 - Redistributor Properties Base Address Register
    pub propbaser: u64, // GICR_PROPBASER

    /// 0x0078 - Redistributor LPI Pending Table Base Address Register
    pub pendbaser: u64, // GICR_PENDBASER

    // 0x0080-0x009C - Reserved
    _reserved4: [u32; 8],

    /// 0x00A0 - Redistributor Invalidate LPI Register
    pub invlpir: u32, // GICR_INVLPIR

    // 0x00A4 - Reserved
    _reserved5: u32,

    // 0x00A8-0x00AC - Reserved
    _reserved6: [u32; 2],

    /// 0x00B0 - Redistributor Invalidate All Register
    pub invallr: u32, // GICR_INVALLR

    // 0x00B4-0x00BC - Reserved
    _reserved7: [u32; 3],

    /// 0x00C0 - Redistributor Synchronize Register
    pub syncr: u32, // GICR_SYNCR

    // 0x00C4-0x00FC - Reserved
    _reserved8: [u32; 15],

    // 0x0100 - IMPLEMENTATION DEFINED registers
    _impl_defined1: u32,

    // 0x0104 - Reserved
    _reserved9: u32,

    // 0x0108 - Reserved
    _reserved10: u32,

    // 0x010C - IMPLEMENTATION DEFINED registers
    _impl_defined2: u32,

    // 0x0110 - IMPLEMENTATION DEFINED registers
    _impl_defined3: u32,

    // 0x0114-0xBFFC - Reserved
    _reserved11: [u32; 12219],

    // 0xC000-0xFFCC - IMPLEMENTATION DEFINED registers
    _impl_defined4: [u32; 4084],
    // 0xFFD0-0xFFE4 - IMPLEMENTATION DEFINED registers
    _impl_defined_id0: [u32; 6],

    /// Distributor Peripheral ID2 Register
    pub pidr2: GicPidr2,

    // 0xFFEC-0xFFFC - IMPLEMENTATION DEFINED registers
    _impl_defined_id1: [u32; 5],
}

pub const GICR_CTLR_OFFSET: usize = 0x0000;
pub const GICR_IIDR_OFFSET: usize = 0x0004;
pub const GICR_TYPER_OFFSET: usize = 0x0008;
pub const GICR_STATUSR_OFFSET: usize = 0x0010;
pub const GICR_WAKER_OFFSET: usize = 0x0014;
pub const GICR_MPAMIDR_OFFSET: usize = 0x0018;
pub const GICR_PARTIDR_OFFSET: usize = 0x001C;
pub const GICR_SETLPIR_OFFSET: usize = 0x0040;
pub const GICR_CLRLPIR_OFFSET: usize = 0x0048;
pub const GICR_PROPBASER_OFFSET: usize = 0x0070;
pub const GICR_PENDBASER_OFFSET: usize = 0x0078;
pub const GICR_INVLPIR_OFFSET: usize = 0x00A0;
pub const GICR_INVALLR_OFFSET: usize = 0x00B0;
pub const GICR_SYNCR_OFFSET: usize = 0x00C0;
pub const GICR_PIDR2_OFFSET: usize = 0xFFE8;

const_assert!(size_of::<GicLpiRedistributor>() == GICR_FRAME_SIZE);

const_assert!(offset_of!(GicLpiRedistributor, ctlr) == GICR_CTLR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, iidr) == GICR_IIDR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, typer) == GICR_TYPER_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, statusr) == GICR_STATUSR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, waker) == GICR_WAKER_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, mpamidr) == GICR_MPAMIDR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, partidr) == GICR_PARTIDR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, setlpir) == GICR_SETLPIR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, clrlpir) == GICR_CLRLPIR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, propbaser) == GICR_PROPBASER_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, pendbaser) == GICR_PENDBASER_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, invlpir) == GICR_INVLPIR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, invallr) == GICR_INVALLR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, syncr) == GICR_SYNCR_OFFSET);
const_assert!(offset_of!(GicLpiRedistributor, pidr2) == GICR_PIDR2_OFFSET);

/// GIC SGI and PPI Redistributor register map
///
/// This struct represents the memory-mapped registers of the ARM GIC Redistributor
/// for SGIs and PPIs. All registers are 32-bit wide and aligned to 4 bytes.
///
/// See "12.10 The GIC Redistributor register map"
#[repr(C, align(0x10000))]
pub struct GicSgiPpiRedistributor {
    // 0x0000-0x007C - Reserved
    _reserved_0000: [u32; 32],

    /// 0x0080 - Interrupt Group Register 0
    pub igroupr0: u32, // GICR_IGROUPR0

    /// 0x0084-0x0088 - Interrupt Group Registers for extended PPI range
    pub igroupr_e: [u32; 2], // GICR_IGROUPR<n>E

    // 0x008C-0x00FC - Reserved
    _reserved_008c: [u32; 29],

    /// 0x0100 - Interrupt Set-Enable Register 0
    pub isenabler0: u32, // GICR_ISENABLER0

    /// 0x0104-0x0108 - Interrupt Set-Enable for extended PPI range
    pub isenabler_e: [u32; 2], // GICR_ISENABLER<n>E

    // 0x010C-0x017C - Reserved
    _reserved_010c: [u32; 29],

    /// 0x0180 - Interrupt Clear-Enable Register 0
    pub icenabler0: u32, // GICR_ICENABLER0

    /// 0x0184-0x0188 - Interrupt Clear-Enable for extended PPI range
    pub icenabler_e: [u32; 2], // GICR_ICENABLER<n>E

    // 0x018C-0x01FC - Reserved
    _reserved_018c: [u32; 29],

    /// 0x0200 - Interrupt Set-Pend Register 0
    pub ispendr0: u32, // GICR_ISPENDR0

    /// 0x0204-0x0208 - Interrupt Set-Pend for extended PPI range
    pub ispendr_e: [u32; 2], // GICR_ISPENDR<n>E

    // 0x020C-0x027C - Reserved
    _reserved_020c: [u32; 29],

    /// 0x0280 - Interrupt Clear-Pend Register 0
    pub icpendr0: u32, // GICR_ICPENDR0

    /// 0x0284-0x0288 - Interrupt Clear-Pend for extended PPI range
    pub icpendr_e: [u32; 2], // GICR_ICPENDR<n>E

    // 0x028C-0x02FC - Reserved
    _reserved_028c: [u32; 29],

    /// 0x0300 - Interrupt Set-Active Register 0
    pub isactiver0: u32, // GICR_ISACTIVER0

    /// 0x0304-0x0308 - Interrupt Set-Active for extended PPI range
    pub isactiver_e: [u32; 2], // GICR_ISACTIVER<n>E

    // 0x030C-0x037C - Reserved
    _reserved_030c: [u32; 29],

    /// 0x0380 - Interrupt Clear-Active Register 0
    pub icactiver0: u32, // GICR_ICACTIVER0

    /// 0x0384-0x0388 - Interrupt Clear-Active for extended PPI range
    pub icactiver_e: [u32; 2], // GICR_ICACTIVER<n>E

    // 0x038C-0x03FC - Reserved
    _reserved_038c: [u32; 29],

    /// 0x0400-0x041C - Interrupt Priority Registers
    pub ipriorityr: [u32; 8], // GICR_IPRIORITYR<n>

    /// 0x0420-0x045C - Interrupt Priority for extended PPI range
    pub ipriorityr_e: [u32; 16], // GICR_IPRIORITYR<n>E

    // 0x0460-0x0BFC - Reserved
    _reserved_0460: [u32; 488],

    /// 0x0C00 - SGI Configuration Register
    pub icfgr0: u32, // GICR_ICFGR0

    /// 0x0C04 - PPI Configuration Register
    pub icfgr1: u32, // GICR_ICFGR1

    /// 0x0C08-0x0C14 - Extended PPI Configuration Register
    pub icfgr_e: [u32; 4], // GICR_ICFGR<n>E

    // 0x0C18-0x0CFC - Reserved
    _reserved_0c18: [u32; 58],

    /// 0x0D00 - Interrupt Group Modifier Register 0
    pub igrpmodr0: u32, // GICR_IGRPMODR0

    /// 0x0D04-0x0D08 - Interrupt Group Modifier for extended PPI range
    pub igrpmodr_e: [u32; 2], // GICR_IGRPMODR<n>E

    // 0x0D0C-0x0DFC - Reserved
    _reserved_0d0c: [u32; 61],

    /// 0x0E00 - Non-Secure Access Control Register
    pub nsacr: u32, // GICR_NSACR

    // 0x0E04-0x0F7C - Reserved
    _reserved_0e04: [u32; 95],

    /// 0x0F80 - Non-maskable Interrupt Register for PPIs and SGIs
    pub inmir0: u32, // GICR_INMIR0

    /// 0x0F84-0x0FFC - Non-maskable Interrupt Registers for Extended PPIs
    pub inmir_e: [u32; 31], // GICR_INMIR<n>E

    // 0x1000-0xBFFC - Reserved
    _reserved_1000: [u32; 11264],

    // 0xC000-0xFFCC - IMPLEMENTATION DEFINED registers
    _impl_defined: [u32; 4084],

    // 0xFFD0-0xFFFC - Reserved
    _reserved_ffd0: [u32; 12],
}

pub const GICR_IGROUPR0_OFFSET: usize = 0x0080;
pub const GICR_IGROUPR_E_OFFSET: usize = 0x0084;
pub const GICR_ISENABLER0_OFFSET: usize = 0x0100;
pub const GICR_ISENABLER_E_OFFSET: usize = 0x0104;
pub const GICR_ICENABLER0_OFFSET: usize = 0x0180;
pub const GICR_ICENABLER_E_OFFSET: usize = 0x0184;
pub const GICR_ISPENDR0_OFFSET: usize = 0x0200;
pub const GICR_ISPENDR_E_OFFSET: usize = 0x0204;
pub const GICR_ICPENDR0_OFFSET: usize = 0x0280;
pub const GICR_ICPENDR_E_OFFSET: usize = 0x0284;
pub const GICR_ISACTIVER0_OFFSET: usize = 0x0300;
pub const GICR_ISACTIVER_E_OFFSET: usize = 0x0304;
pub const GICR_ICACTIVER0_OFFSET: usize = 0x0380;
pub const GICR_ICACTIVER_E_OFFSET: usize = 0x0384;
pub const GICR_IPRIORITYR_OFFSET: usize = 0x0400;
pub const GICR_IPRIORITYR_E_OFFSET: usize = 0x0420;
pub const GICR_ICFGR0_OFFSET: usize = 0x0C00;
pub const GICR_ICFGR1_OFFSET: usize = 0x0C04;
pub const GICR_ICFGR_E_OFFSET: usize = 0x0C08;
pub const GICR_IGRPMODR0_OFFSET: usize = 0x0D00;
pub const GICR_IGRPMODR_E_OFFSET: usize = 0x0D04;
pub const GICR_NSACR_OFFSET: usize = 0x0E00;
pub const GICR_INMIR0_OFFSET: usize = 0x0F80;
pub const GICR_INMIR_E_OFFSET: usize = 0x0F84;

const_assert!(size_of::<GicSgiPpiRedistributor>() == 0x10000);

const_assert!(offset_of!(GicSgiPpiRedistributor, igroupr0) == GICR_IGROUPR0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, igroupr_e) == GICR_IGROUPR_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, isenabler0) == GICR_ISENABLER0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, isenabler_e) == GICR_ISENABLER_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icenabler0) == GICR_ICENABLER0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icenabler_e) == GICR_ICENABLER_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, ispendr0) == GICR_ISPENDR0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, ispendr_e) == GICR_ISPENDR_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icpendr0) == GICR_ICPENDR0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icpendr_e) == GICR_ICPENDR_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, isactiver0) == GICR_ISACTIVER0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, isactiver_e) == GICR_ISACTIVER_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icactiver0) == GICR_ICACTIVER0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icactiver_e) == GICR_ICACTIVER_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, ipriorityr) == GICR_IPRIORITYR_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, ipriorityr_e) == GICR_IPRIORITYR_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icfgr0) == GICR_ICFGR0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icfgr1) == GICR_ICFGR1_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, icfgr_e) == GICR_ICFGR_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, igrpmodr0) == GICR_IGRPMODR0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, igrpmodr_e) == GICR_IGRPMODR_E_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, nsacr) == GICR_NSACR_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, inmir0) == GICR_INMIR0_OFFSET);
const_assert!(offset_of!(GicSgiPpiRedistributor, inmir_e) == GICR_INMIR_E_OFFSET);

#[repr(C, align(0x10000))]
pub struct GicRedistributor {
    pub lpi: GicLpiRedistributor,
    pub sgi_ppi: GicSgiPpiRedistributor,
}

const_assert!(size_of::<GicRedistributor>() == GICR_SIZE);

const_assert!(offset_of!(GicRedistributor, lpi) == 0);
const_assert!(offset_of!(GicRedistributor, sgi_ppi) == GICR_FRAME_SIZE);

/// GICv3 intrerface
pub struct Gicv3<'a> {
    /// Ditributor
    gicd: &'a mut GicDistributor,
    /// Redistibutors for each CPU
    gicr: &'a mut [GicRedistributor],
}

/// GICv3
///
/// Initalization and configurations are described in "4. Configuring the GIC" of
/// [GICv3 and GICv4 Software Overview](https://developer.arm.com/documentation/dai0492/b/)
impl<'a> Gicv3<'a> {
    /// Initialize the GICv3 interface
    ///
    /// # Safety
    ///
    /// The pointers come from a trusted location and are not aliased.
    pub unsafe fn new(
        gicd_base: *mut GicDistributor,
        gicr_base: *mut GicRedistributor,
        num_cpus: usize,
    ) -> Self {
        let gicd = unsafe { gicd_base.as_mut().expect("non NULL GICD") };
        let gicr = unsafe { core::slice::from_raw_parts_mut(gicr_base, num_cpus) };

        let mut gic = Self { gicd, gicr };

        let gicd_ver = gic.gicd.pidr2.gic_version();
        assert_eq!(gicd_ver, 3, "Expected GIC v3, got {gicd_ver}");

        for (i, r) in gic.gicr.iter().enumerate() {
            let r_ver = r.lpi.pidr2.gic_version();
            assert_eq!(r_ver, 3, "Expected GICR v3, got {r_ver} on CPU {i}");
        }

        gic.init_gicd();
        gic.init_gicr();
        gic.init_icc();

        gic
    }

    /// Initialize the distributor, route all SPIs to the BSP
    fn init_gicd(&mut self) {
        self.gicd.ctlr.reset();
        self.gicd.ctlr.wait_pending_write();

        // Mask and clear all SPIs
        let max_spi = self.spi_lines();
        for i in 1..max_spi / 32 {
            self.gicd.icenabler[i] = !0;
            self.gicd.icpendr[i] = !0;
            self.gicd.igroupr[i] = !0;
            self.gicd.igrpmodr[i] = !0;
        }
        self.gicd.ctlr.wait_pending_write();

        self.gicd.ctlr.set_enable_grp0(1);
        self.gicd.ctlr.set_enable_grp1_ns(1);
        self.gicd.ctlr.set_are_ns(1);

        unsafe { core::arch::asm!("isb sy", options(nostack)) };

        // CPU 0, affinity 0.0.0.0
        for i in 32..max_spi {
            self.gicd.irouter[i] = 0;
        }
        self.gicd.ctlr.wait_pending_write();

        unsafe { core::arch::asm!("isb sy", options(nostack)) };
    }

    /// Initialize the redistributor
    fn init_gicr(&mut self) {}

    /// Initialize the control interface to the CPU
    /// through the ICC_* system registers
    fn init_icc(&mut self) {}

    pub fn gicd(&self) -> &GicDistributor {
        self.gicd
    }

    pub fn gicr(&self) -> &[GicRedistributor] {
        self.gicr
    }

    pub fn spi_lines(&self) -> usize {
        32 * (self.gicd.typer.it_lines() + 1) as usize
    }

    pub fn lpi_lines(&self) -> usize {
        let intid = self.gicd.typer.id_bits();
        if intid <= 14 {
            return 0;
        }

        1 << (self.gicd.typer.lpi_lines() + 1)
    }
}

pub fn valid_spi_id(id: usize) -> bool {
    (32..1019).contains(&id)
}
