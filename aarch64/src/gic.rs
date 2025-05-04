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

use crate::dev_registrer::DeviceRegister;
use crate::dev_registrer::DeviceRegisterArray;
use crate::dev_registrer::DeviceRegisterArraySpec;
use crate::dev_registrer::DeviceRegisterSpec;
use bitfield_struct::bitfield;

pub const GICD_SIZE: usize = 0x10000;

// GIC registers, "12.9 The GIC Distributor register descriptions"

// GIC Distributor register map
// See "12.8 The GIC Distributor register map".

/// 0x0000 - Distributor Control Register (GICD_CTLR)
///
/// Controls overall operation of the Distributor.
/// The reset value is implementation defined.
pub const GICD_CTLR_OFFSET: usize = 0x0000; // u32

/// 0x0004 - Interrupt Controller Type Register (GICD_TYPER)
///
/// Provides information about the configuration of the GIC.
/// Read-only, implementation defined.
pub const GICD_TYPER_OFFSET: usize = 0x0004; // u32

/// 0x0008 - Distributor Implementer Identification Register (GICD_IIDR)
///
/// Identifies the implementer of the GIC.
/// Read-only, implementation defined.
pub const GICD_IIDR_OFFSET: usize = 0x0008; // u32

/// 0x000C - Interrupt Controller Type Register 2 (GICD_TYPER2)
///
/// Additional type information about the GIC.
/// Read-only, implementation defined.
pub const GICD_TYPER2_OFFSET: usize = 0x000C; // u32

/// 0x0010 - Error Reporting Status Register (optional) (GICD_STATUSR)
///
/// Reports error conditions in the Distributor.
/// Reset value: 0x00000000
pub const GICD_STATUSR_OFFSET: usize = 0x0010; // u32

/// 0x0040 - Set SPI Register (Non-secure) (GICD_SETSPI_NSR)
///
/// Writing to this register sets the corresponding SPI interrupt
/// pending state in the non-secure state.
pub const GICD_SETSPI_NSR_OFFSET: usize = 0x0040; // u32

/// 0x0048 - Clear SPI Register (Non-secure) (GICD_CLRSPI_NSR)
///
/// Writing to this register clears the corresponding SPI interrupt
/// pending state in the non-secure state.
pub const GICD_CLRSPI_NSR_OFFSET: usize = 0x0048; // u32

/// 0x0050 - Set SPI Register (Secure) (GICD_SETSPI_SR)
///
/// Writing to this register sets the corresponding SPI interrupt
/// pending state in the secure state.
pub const GICD_SETSPI_SR_OFFSET: usize = 0x0050; // u32

/// 0x0058 - Clear SPI Register (Secure) (GICD_CLRSPI_SR)
///
/// Writing to this register clears the corresponding SPI interrupt
/// pending state in the secure state.
pub const GICD_CLRSPI_SR_OFFSET: usize = 0x0058; // u32

/// 0x0080-0x00FC - Interrupt Group Registers (GICD_IGROUPR<n>)
///
/// Configures interrupts as Group 0 or Group 1.
/// Reset value: implementation defined.
pub const GICD_IGROUPR_OFFSET: usize = 0x0080; // [u32; 32]

/// 0x0100-0x017C - Interrupt Set-Enable Registers (GICD_ISENABLER<n>)
///
/// Enables forwarding of interrupts to CPU interfaces.
/// Reset value: implementation defined.
pub const GICD_ISENABLER_OFFSET: usize = 0x0100; // [u32; 32]

/// 0x0180-0x01FC - Interrupt Clear-Enable Registers (GICD_ICENABLER<n>)
///
/// Disables forwarding of interrupts to CPU interfaces.
/// Reset value: implementation defined.
pub const GICD_ICENABLER_OFFSET: usize = 0x0180; // [u32; 32]

/// 0x0200-0x027C - Interrupt Set-Pending Registers (GICD_ISPENDR<n>)
///
/// Sets the pending state of interrupts.
/// Reset value: 0x00000000
pub const GICD_ISPENDR_OFFSET: usize = 0x0200; // [u32; 32]

/// 0x0280-0x02FC - Interrupt Clear-Pending Registers (GICD_ICPENDR<n>)
///
/// Clears the pending state of interrupts.
/// Reset value: 0x00000000
pub const GICD_ICPENDR_OFFSET: usize = 0x0280; // [u32; 32]

/// 0x0300-0x037C - Interrupt Set-Active Registers (GICD_ISACTIVER<n>)
///
/// Sets the active state of interrupts.
/// Reset value: 0x00000000
pub const GICD_ISACTIVER_OFFSET: usize = 0x0300; // [u32; 32]

/// 0x0380-0x03FC - Interrupt Clear-Active Registers (GICD_ICACTIVER<n>)
///
/// Clears the active state of interrupts.
/// Reset value: 0x00000000
pub const GICD_ICACTIVER_OFFSET: usize = 0x0380; // [u32; 32]

/// 0x0400-0x07F8 - Interrupt Priority Registers (GICD_IPRIORITYR<n>)
///
/// Configures the priority of each interrupt.
/// Reset value: 0x00000000
pub const GICD_IPRIORITYR_OFFSET: usize = 0x0400; // [u32; 256]

/// 0x0800-0x081C - Interrupt Processor Targets Registers (GICD_ITARGETSR<n>)
///
/// Configures which CPUs receive each interrupt. Read-only, implementation defined.
/// RES0 when affinity routing is enabled.
pub const GICD_ITARGETSR_OFFSET: usize = 0x0800; // [u32; 8]

/// 0x0C00-0x0CFC - Interrupt Configuration Registers (GICD_ICFGR<n>)
///
/// Configures interrupts as level-sensitive or edge-triggered.
/// Reset value: implementation defined.
pub const GICD_ICFGR_OFFSET: usize = 0x0C00; // [u32; 64]

/// 0x0D00-0x0D7C - Interrupt Group Modifier Registers (GICD_IGRPMODR<n>)
///
/// Modifies interrupt group behavior.
/// Reset value: 0x00000000
pub const GICD_IGRPMODR_OFFSET: usize = 0x0D00; // [u32; 32]

/// 0x0E00-0x0EFC - Non-secure Access Control Registers (GICD_NSACR<n>)
///
/// Controls non-secure access to secure interrupts.
/// Reset value: 0x00000000
pub const GICD_NSACR_OFFSET: usize = 0x0E00; // [u32; 64]

/// 0x0F00 - Software Generated Interrupt Register (GICD_SGIR)
///
/// Generates software interrupts.
/// RES0 when affinity routing is enabled.
pub const GICD_SGIR_OFFSET: usize = 0x0F00; // u32

/// 0x0F10-0x0F1C - SGI Clear-Pending Registers (GICD_CPENDSGIR<n>)
///
/// Clears pending state of SGIs. Reset value: 0x00000000
/// RES0 when affinity routing is enabled.
pub const GICD_CPENDSGIR_OFFSET: usize = 0x0F10; // [u32; 4]

/// 0x0F20-0x0F2C - SGI Set-Pending Registers (GICD_SPENDSGIR<n>)
///
/// Sets pending state of SGIs. Reset value: 0x00000000
/// RES0 when affinity routing is enabled.
pub const GICD_SPENDSGIR_OFFSET: usize = 0x0F20; // [u32; 4]

/// 0x0F80-0x0FFC - Non-maskable Interrupt Registers (GICD_INMIR<n>)
///
/// Controls non-maskable interrupts.
/// Reset value: 0x00000000
pub const GICD_INMIR_OFFSET: usize = 0x0F80; // [u32; 32]

// Extended SPI registers (0x1000 onwards)

/// 0x1000-0x107C - Interrupt Group Registers for extended SPI range (GICD_IGROUPR<n>E)
///
/// Configures extended SPIs as Group 0 or Group 1.
/// Reset value: 0x00000000
pub const GICD_IGROUPR_E_OFFSET: usize = 0x1000; // [u32; 32]

/// 0x1200-0x127C - Interrupt Set-Enable for extended SPI range (GICD_ISENABLER<n>E)
///
/// Enables forwarding of extended SPI interrupts.
/// Reset value: implementation defined.
pub const GICD_ISENABLER_E_OFFSET: usize = 0x1200; // [u32; 32]

/// 0x1400-0x147C - Interrupt Clear-Enable for extended SPI range (GICD_ICENABLER<n>E)
///
/// Disables forwarding of extended SPI interrupts.
/// Reset value: implementation defined.
pub const GICD_ICENABLER_E_OFFSET: usize = 0x1400; // [u32; 32]

/// 0x1600-0x167C - Interrupt Set-Pend for extended SPI range (GICD_ISPENDR<n>E)
///
/// Sets the pending state of extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_ISPENDR_E_OFFSET: usize = 0x1600; // [u32; 32]

/// 0x1800-0x187C - Interrupt Clear-Pend for extended SPI range (GICD_ICPENDR<n>E)
///
/// Clears the pending state of extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_ICPENDR_E_OFFSET: usize = 0x1800; // [u32; 32]

/// 0x1A00-0x1A7C - Interrupt Set-Active for extended SPI range (GICD_ISACTIVER<n>E)
///
/// Sets the active state of extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_ISACTIVER_E_OFFSET: usize = 0x1A00; // [u32; 32]

/// 0x1C00-0x1C7C - Interrupt Clear-Active for extended SPI range (GICD_ICACTIVER<n>E)
///
/// Clears the active state of extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_ICACTIVER_E_OFFSET: usize = 0x1C00; // [u32; 32]

/// 0x2000-0x23FC - Interrupt Priority for extended SPI range (GICD_IPRIORITYR<n>E)
///
/// Configures the priority of extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_IPRIORITYR_E_OFFSET: usize = 0x2000; // [u32; 256]

/// 0x3000-0x30FC - Extended SPI Configuration Register (GICD_ICFGR<n>E)
///
/// Configures extended SPI interrupts as level-sensitive or edge-triggered.
/// Reset value: implementation defined.
pub const GICD_ICFGR_E_OFFSET: usize = 0x3000; // [u32; 64]

/// 0x3400-0x347C - Interrupt Group Modifier for extended SPI range (GICD_IGRPMODR<n>E)
///
/// Modifies extended SPI interrupt group behavior.
/// Reset value: 0x00000000
pub const GICD_IGRPMODR_E_OFFSET: usize = 0x3400; // [u32; 32]

/// 0x3600-0x36FC - Non-secure Access Control Registers for extended SPI range (GICD_NSACR<n>E)
///
/// Controls non-secure access to secure extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_NSACR_E_OFFSET: usize = 0x3600; // [u32; 64]

/// 0x3B00-0x3B7C - Non-maskable Interrupt Registers for Extended SPIs (GICD_INMIR<n>Eg)
///
/// Controls non-maskable extended SPI interrupts.
/// Reset value: 0x00000000
pub const GICD_INMIR_E_OFFSET: usize = 0x3B00; // [u32; 32]

/// 0x6100-0x7FD8 - Interrupt Routing Registers (GICD_IROUTER<n>)
///
/// Configures interrupt routing for affinity-based systems.
/// Reset value: 0x00000000
pub const GICD_IROUTER_OFFSET: usize = 0x6100; // [u32; 1984]

/// 0x8000-0x9FFC - Interrupt Routing Registers for extended SPI range (GICD_IROUTER<n>E)
///
/// Configures interrupt routing for extended SPI interrupts in affinity-based systems.
/// Reset value: 0x00000000
pub const GICD_IROUTER_E_OFFSET: usize = 0x8000; // [u32; 2048]

/// 0xFFE8 - Distributor Peripheral ID2 Register (GICD_PIDR2)
///
/// Provides version data about the distributor.
///
pub const GICD_PIDR2_OFFSET: usize = 0xFFE8; // u32

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

impl DeviceRegisterSpec for GicdCtrl {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_CTLR_OFFSET;
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

impl DeviceRegisterSpec for GicdTyper {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_TYPER_OFFSET;
}

#[bitfield(u32)]
/// Peripheral ID2 Register
pub struct GicdPidr2 {
    #[bits(4)]
    pub _impl_def0: u32,
    #[bits(4)]
    pub gic_version: u32,
    #[bits(24)]
    pub _impl_def1: u32,
}

impl DeviceRegisterSpec for GicdPidr2 {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_PIDR2_OFFSET;
}

/// Clear enabled interrupts
#[bitfield(u32)]
pub struct GicdIcenabler {
    pub icenable: u32,
}

impl DeviceRegisterSpec for GicdIcenabler {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_ICENABLER_OFFSET;
}

impl DeviceRegisterArraySpec for GicdIcenabler {
    const COUNT: usize = 32;
}

/// Clear pending interrupts
#[bitfield(u32)]
pub struct GicdIcpendr {
    pub icpend: u32,
}

impl DeviceRegisterSpec for GicdIcpendr {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_ICPENDR_OFFSET;
}

impl DeviceRegisterArraySpec for GicdIcpendr {
    const COUNT: usize = 32;
}

/// Interrupt Group Registers
#[bitfield(u32)]

pub struct GicdIgroupr {
    pub igroup: u32,
}

impl DeviceRegisterSpec for GicdIgroupr {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_IGROUPR_OFFSET;
}

impl DeviceRegisterArraySpec for GicdIgroupr {
    const COUNT: usize = 32;
}

/// Interrupt Group Modifier Registers
#[bitfield(u32)]
pub struct GicdIgrpmodr {
    pub igrpmod: u32,
}

impl DeviceRegisterSpec for GicdIgrpmodr {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_IGRPMODR_OFFSET;
}

impl DeviceRegisterArraySpec for GicdIgrpmodr {
    const COUNT: usize = 32;
}

/// Interrupt Routing Registers
#[bitfield(u32)]
pub struct GicdIrouter {
    pub iroute: u32,
}

impl DeviceRegisterSpec for GicdIrouter {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICD_IROUTER_OFFSET;
}

impl DeviceRegisterArraySpec for GicdIrouter {
    const COUNT: usize = 1984;
}
// GICR registers, "12.11 The GIC Redistributor register descriptions"

// GIC physical LPI Redistributor register map

pub const GICR_SIZE: usize = 0x20000;
pub const GICR_FRAME_SIZE: usize = 0x10000;

/// 0x0000 - Redistributor Control Register (GICR_CTLR)
pub const GICR_CTLR_OFFSET: usize = 0x0000;

/// 0x0004 - Implementer Identification Register (GICR_IIDR)
pub const GICR_IIDR_OFFSET: usize = 0x0004;

/// 0x0008 - Redistributor Type Register (GICR_TYPER)
pub const GICR_TYPER_OFFSET: usize = 0x0008;

/// 0x0010 - Error Reporting Status Register (optional) (GICR_STATUSR)
pub const GICR_STATUSR_OFFSET: usize = 0x0010;

/// 0x0014 - Redistributor Wake Register (GICR_WAKER)
pub const GICR_WAKER_OFFSET: usize = 0x0014;

/// 0x0018 - Report maximum PARTID and PMG Register (GICR_MPAMIDR)
pub const GICR_MPAMIDR_OFFSET: usize = 0x0018;

/// 0x001C - Set PARTID and PMG Register (GICR_PARTIDR)
pub const GICR_PARTIDR_OFFSET: usize = 0x001C;

/// 0x0040 - Set LPI Pending Register (GICR_SETLPIR)
pub const GICR_SETLPIR_OFFSET: usize = 0x0040;

/// 0x0048 - Clear LPI Pending Register (GICR_CLRLPIR)
pub const GICR_CLRLPIR_OFFSET: usize = 0x0048;

/// 0x0070 - Redistributor Properties Base Address Register (GICR_PROPBASER)
pub const GICR_PROPBASER_OFFSET: usize = 0x0070;

/// 0x0078 - Redistributor LPI Pending Table Base Address Register (GICR_PENDBASER)
pub const GICR_PENDBASER_OFFSET: usize = 0x0078;

/// 0x00A0 - Redistributor Invalidate LPI Register (GICR_INVLPIR)
pub const GICR_INVLPIR_OFFSET: usize = 0x00A0;

/// 0x00B0 - Redistributor Invalidate All Register (GICR_INVALLR)
pub const GICR_INVALLR_OFFSET: usize = 0x00B0;

/// 0x00C0 - Redistributor Synchronize Register (GICR_SYNCR)
pub const GICR_SYNCR_OFFSET: usize = 0x00C0;

/// Distributor Peripheral ID2 (GICR_PIDR2)
pub const GICR_PIDR2_OFFSET: usize = 0xFFE8;

/// GICR control
#[bitfield(u32)]
pub struct GicrCtlr {
    #[bits(1)]
    pub enable_lpis: u32,
    #[bits(1)]
    pub ces: u32,
    #[bits(1)]
    pub ir: u32,
    #[bits(1)]
    pub reg_write_pending: u32,
    #[bits(20)]
    _res0: u32,
    #[bits(1)]
    pub dpg0: u32,
    #[bits(1)]
    pub dpg1ns: u32,
    #[bits(1)]
    pub dpg1s: u32,
    #[bits(4)]
    _res1: u32,
    #[bits(1)]
    pub upstream_write_pending: u32,
}

impl DeviceRegisterSpec for GicrCtlr {
    type Raw = u32;
    type Value = GicrCtlr;
    const OFFSET: usize = GICR_CTLR_OFFSET;
}

/// GICR Identification register
#[bitfield(u32)]
pub struct GicrIidr {
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

impl DeviceRegisterSpec for GicrIidr {
    type Raw = u32;
    type Value = GicrIidr;
    const OFFSET: usize = GICR_IIDR_OFFSET;
}

/// GICR Type register
#[bitfield(u64)]
pub struct GicrTyper {
    #[bits(1)]
    pub plpis: u64,
    #[bits(1)]
    pub vlpis: u64,
    #[bits(1)]
    pub dirty: u64,
    #[bits(1)]
    pub direct_lpi: u64,
    #[bits(1)]
    pub last: u64,
    #[bits(1)]
    pub dpgs: u64,
    #[bits(1)]
    pub mpam: u64,
    #[bits(1)]
    pub rvpeid: u64,
    #[bits(16)]
    pub processor_number: u64,
    #[bits(2)]
    pub common_lpi_aff: u64,
    #[bits(1)]
    pub vsgi: u64,
    #[bits(5)]
    pub ppi_num: u64,
    #[bits(8)]
    pub aff0: u64,
    #[bits(8)]
    pub aff1: u64,
    #[bits(8)]
    pub aff2: u64,
    #[bits(8)]
    pub aff3: u64,
}

impl DeviceRegisterSpec for GicrTyper {
    type Raw = u64;
    type Value = GicrTyper;
    const OFFSET: usize = GICR_TYPER_OFFSET;
}

/// GICR Wake register
#[bitfield(u32)]
pub struct GicrWaker {
    #[bits(1)]
    pub _impl_def0: u32,
    #[bits(1)]
    pub processor_sleep: u32,
    #[bits(1)]
    pub children_asleep: u32,
    #[bits(28)]
    _res0: u32,
    #[bits(1)]
    pub _impl_def1: u32,
}

impl DeviceRegisterSpec for GicrWaker {
    type Raw = u32;
    type Value = GicrWaker;
    const OFFSET: usize = GICR_WAKER_OFFSET;
}

#[bitfield(u32)]
/// Peripheral ID2 Register
pub struct GicrPidr2 {
    #[bits(4)]
    pub _impl_def0: u32,
    #[bits(4)]
    pub gic_version: u32,
    #[bits(24)]
    pub _impl_def1: u32,
}

impl DeviceRegisterSpec for GicrPidr2 {
    type Raw = u32;
    type Value = Self;
    const OFFSET: usize = GICR_PIDR2_OFFSET;
}

// GIC SGI and PPI Redistributor register map
// See "12.10 The GIC Redistributor register map"

/// 0x0080 - Interrupt Group Register 0 (GICR_IGROUPR0)
pub const GICR_IGROUPR0_OFFSET: usize = GICR_FRAME_SIZE + 0x0080; // u32

/// 0x0084-0x0088 - Interrupt Group Registers for extended PPI range (GICR_IGROUPR<n>E)
pub const GICR_IGROUPR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0084; // [u32; 2]

/// 0x0100 - Interrupt Set-Enable Register 0 (GICR_ISENABLER0)
pub const GICR_ISENABLER0_OFFSET: usize = GICR_FRAME_SIZE + 0x0100; // u32

/// 0x0104-0x0108 - Interrupt Set-Enable for extended PPI range (GICR_ISENABLER<n>E)
pub const GICR_ISENABLER_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0104; // [u32; 2]

/// 0x0180 - Interrupt Clear-Enable Register 0 (GICR_ICENABLER0)
pub const GICR_ICENABLER0_OFFSET: usize = GICR_FRAME_SIZE + 0x0180; // u32

/// 0x0184-0x0188 - Interrupt Clear-Enable for extended PPI range (GICR_ICENABLER<n>E)
pub const GICR_ICENABLER_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0184; // [u32; 2]

/// 0x0200 - Interrupt Set-Pend Register 0 (GICR_ISPENDR0)
pub const GICR_ISPENDR0_OFFSET: usize = GICR_FRAME_SIZE + 0x0200; // u32

/// 0x0204-0x0208 - Interrupt Set-Pend for extended PPI range (GICR_ISPENDR<n>E)
pub const GICR_ISPENDR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0204; // [u32; 2]

/// 0x0280 - Interrupt Clear-Pend Register 0 (GICR_ICPENDR0)
pub const GICR_ICPENDR0_OFFSET: usize = GICR_FRAME_SIZE + 0x0280; // u32

/// 0x0284-0x0288 - Interrupt Clear-Pend for extended PPI range (GICR_ICPENDR<n>E)
pub const GICR_ICPENDR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0284; // [u32; 2]

/// 0x0300 - Interrupt Set-Active Register 0 (GICR_ISACTIVER0)
pub const GICR_ISACTIVER0_OFFSET: usize = GICR_FRAME_SIZE + 0x0300; // u32

/// 0x0304-0x0308 - Interrupt Set-Active for extended PPI range (GICR_ISACTIVER<n>E)
pub const GICR_ISACTIVER_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0304; // [u32; 2]

/// 0x0380 - Interrupt Clear-Active Register 0 (GICR_ICACTIVER0)
pub const GICR_ICACTIVER0_OFFSET: usize = GICR_FRAME_SIZE + 0x0380; // u32

/// 0x0384-0x0388 - Interrupt Clear-Active for extended PPI range (GICR_ICACTIVER<n>E)
pub const GICR_ICACTIVER_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0384; // [u32; 2]

/// 0x0400-0x041C - Interrupt Priority Registers (GICR_IPRIORITYR<n>)
///
/// - GICR_IPRIORITYR0-GICR_IPRIORITYR3 store the priority of SGIs.
/// - GICR_IPRIORITYR4-GICR_IPRIORITYR7 store the priority of PPIs.
///
/// Interrupt priority value from an IMPLEMENTATION DEFINED range,
/// takes 8 bits. Lower priority values correspond to greater priority
/// of the interrupt. For an INTID configured as non-maskable, this field is RES0.
pub const GICR_IPRIORITYR_OFFSET: usize = GICR_FRAME_SIZE + 0x0400; // [u32; 8]

/// 0x0420-0x045C - Interrupt Priority for extended PPI range (GICR_IPRIORITYR<n>E)
pub const GICR_IPRIORITYR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0420; // [u32; 16]

/// 0x0C00 - SGI Configuration Register (GICR_ICFGR0)
pub const GICR_ICFGR0_OFFSET: usize = GICR_FRAME_SIZE + 0x0C00; // u32

/// 0x0C04 - PPI Configuration Register (GICR_ICFGR1)
pub const GICR_ICFGR1_OFFSET: usize = GICR_FRAME_SIZE + 0x0C04; // u32

/// 0x0C08-0x0C14 - Extended PPI Configuration Register (GICR_ICFGR<n>E)
pub const GICR_ICFGR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0C08; // [u32; 4]

/// 0x0D00 - Interrupt Group Modifier Register 0 (GICR_IGRPMODR0)
pub const GICR_IGRPMODR0_OFFSET: usize = GICR_FRAME_SIZE + 0x0D00; // u32

/// 0x0D04-0x0D08 - Interrupt Group Modifier for extended PPI range (GICR_IGRPMODR<n>E)
pub const GICR_IGRPMODR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0D04; // [u32; 2]

/// 0x0E00 - Non-Secure Access Control Register (GICR_NSACR)
pub const GICR_NSACR_OFFSET: usize = GICR_FRAME_SIZE + 0x0E00; // u32

/// 0x0F80 - Non-maskable Interrupt Register for PPIs and SGIs (GICR_INMIR0)
pub const GICR_INMIR0_OFFSET: usize = GICR_FRAME_SIZE + 0x0F80; // u32

/// 0x0F84-0x0FFC - Non-maskable Interrupt Registers for Extended PPIs (GICR_INMIR<n>E)
pub const GICR_INMIR_E_OFFSET: usize = GICR_FRAME_SIZE + 0x0F84; // [u32; 31]

#[bitfield(u32)]
pub struct GicrIpriorityr {
    p0: u8,
    p1: u8,
    p2: u8,
    p3: u8,
}

impl DeviceRegisterSpec for GicrIpriorityr {
    type Raw = u32;
    type Value = GicrIpriorityr;
    const OFFSET: usize = GICR_IPRIORITYR_OFFSET;
}

impl DeviceRegisterArraySpec for GicrIpriorityr {
    const COUNT: usize = 8;
}

/// GIC version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GicVersion {
    /// Version 3: GICD and GICR with two frames per CPU.
    GicV3,
    /// Version 4: GICD and GICR with four frames per CPU.
    GicV4,
}

/// GIC intrerface
pub struct Gic {
    gicd_base: usize,
    gicr_base: usize,
    version: GicVersion,
    max_spi: usize,
    num_cpus: usize,
    redist_size: usize,
}

/// GIC
///
/// Initalization and configurations are described in "4. Configuring the GIC" of
/// [GICv3 and GICv4 Software Overview](https://developer.arm.com/documentation/dai0492/b/)
///
/// Might be better to refactor to use type states.
impl Gic {
    /// Initialize the GIC interface
    pub fn new(gicd_base: usize, gicr_base: usize, num_cpus: usize) -> Self {
        // Run some basic (in)sanity checks

        let gicd_pidr2 = DeviceRegister::<GicdPidr2>::new(gicd_base);
        let gicd_ver = gicd_pidr2.load().gic_version();
        assert!(
            gicd_ver == 3 || gicd_ver == 4,
            "Expected GIC v3 or GIC v4, got {gicd_ver}"
        );

        let (redist_size, version) = if gicd_ver == 3 {
            // Got LPI and the SGI+PPI frames
            (2 * GICR_FRAME_SIZE, GicVersion::GicV3)
        } else if gicd_ver == 4 {
            // The redistributor in GICv4 has two additional frames: VLPI and Reserved
            (4 * GICR_FRAME_SIZE, GicVersion::GicV4)
        } else {
            unreachable!();
        };

        for i in 0..num_cpus {
            let gicr_pidr2 = DeviceRegister::<GicrPidr2>::new(gicr_base + i * redist_size);
            let gicr_ver = gicr_pidr2.load().gic_version();
            assert!(
                gicr_ver == 3 || gicr_ver == 4,
                "Expected GIC v3 or GIC v4, got {gicr_ver}"
            );

            let gicr_typer = DeviceRegister::<GicrTyper>::new(gicr_base + i * redist_size);
            let vlpis = gicr_typer.load().vlpis();
            assert!(
                vlpis == 0 || (gicr_ver == 4 && gicd_ver == 4),
                "Expected VLPIs in GIC v4, CPU {i}"
            );
        }

        // Initialize the instance

        let gicd_typer = DeviceRegister::<GicdTyper>::new(gicd_base);
        let max_spi = (32 * gicd_typer.load().it_lines() + 1) as usize;

        Self {
            gicd_base,
            gicr_base,
            version,
            max_spi,
            num_cpus,
            redist_size,
        }
    }

    /// Initialize the distributor, route all SPIs to the BSP.
    pub fn init_gicd(&mut self) {
        let mut gicd_ctrl = DeviceRegister::<GicdCtrl>::new(self.gicd_base);

        // Reset
        gicd_ctrl.store(GicdCtrl::new());
        while gicd_ctrl.load().reg_write_pending() != 0 {
            unsafe { core::arch::asm!("yield", options(nostack)) }
        }

        // Mask and clear all SPIs
        let max_spi = self.max_spi;

        DeviceRegisterArray::<GicdIcenabler>::new(self.gicd_base)
            .fill(1..max_spi / 32, GicdIcenabler::from(!0));
        DeviceRegisterArray::<GicdIcpendr>::new(self.gicd_base)
            .fill(1..max_spi / 32, GicdIcpendr::from(!0));
        DeviceRegisterArray::<GicdIgroupr>::new(self.gicd_base)
            .fill(1..max_spi / 32, GicdIgroupr::from(!0));
        DeviceRegisterArray::<GicdIgrpmodr>::new(self.gicd_base)
            .fill(1..max_spi / 32, GicdIgrpmodr::from(!0));
        while gicd_ctrl.load().reg_write_pending() != 0 {
            unsafe { core::arch::asm!("yield", options(nostack)) }
        }

        gicd_ctrl.store(
            GicdCtrl::new()
                .with_enable_grp0(1)
                .with_enable_grp1_ns(1)
                .with_are_ns(1),
        );
        while gicd_ctrl.load().reg_write_pending() != 0 {
            unsafe { core::arch::asm!("yield", options(nostack)) }
        }

        unsafe { core::arch::asm!("isb sy", options(nostack)) };

        // CPU 0, affinity 0.0.0.0
        DeviceRegisterArray::<GicdIrouter>::new(self.gicd_base)
            .fill(32..max_spi, GicdIrouter::from(0));
        while gicd_ctrl.load().reg_write_pending() != 0 {
            unsafe { core::arch::asm!("yield", options(nostack)) }
        }

        unsafe { core::arch::asm!("isb sy", options(nostack)) };
    }

    /// Wake up the CPU and initialize its redistributor.
    pub fn wakeup_cpu_and_init_gicr(&mut self, cpu: usize) {
        let gicr_base = self.gicr_base + cpu * self.redist_size;

        // Wake up the CPU

        let mut waker = DeviceRegister::<GicrWaker>::new(gicr_base);
        waker.store(waker.load().with_processor_sleep(0));
        while waker.load().children_asleep() != 0 {
            unsafe { core::arch::asm!("yield", options(nostack)) }
        }

        // Configure interrupts

        let mut ipriorityr = DeviceRegisterArray::<GicrIpriorityr>::new(gicr_base);

        // SGI priorities, implementation defined
        let sgi_prio = GicrIpriorityr::new()
            .with_p0(0x90)
            .with_p1(0x90)
            .with_p2(0x90)
            .with_p3(0x90);
        ipriorityr.fill(0..4, sgi_prio);

        // PPI priorities, implementation defined
        let ppi_prio = GicrIpriorityr::new()
            .with_p0(0xa0)
            .with_p1(0xa0)
            .with_p2(0xa0)
            .with_p3(0xa0);
        ipriorityr.fill(4..8, ppi_prio);

        // // Disable forwarding all PPI and SGI to the CPU interface
        // sgi_ppi.icenabler0 = 0;
        // sgi_ppi.isenabler0 = 0;

        // // Set SGI and PPI as non-secure group 1
        // sgi_ppi.igroupr0 = 0xffff_ffff;

        let gicr_ctrl = DeviceRegister::<GicrCtlr>::new(gicr_base);
        while gicr_ctrl.load().reg_write_pending() != 0 {
            unsafe { core::arch::asm!("yield", options(nostack)) }
        }

        unsafe { core::arch::asm!("isb sy", options(nostack)) };
    }

    //     fn enable_interrupt(&mut self, irq_num: usize, enable: bool) {
    //         if enable {
    //             self.sgi_ppi.icenabler0 &= !(1 << irq_num);
    //             self.sgi_ppi.isenabler0 |= 1 << irq_num;
    //         } else {
    //             self.sgi_ppi.isenabler0 &= !(1 << irq_num);
    //             self.sgi_ppi.icenabler0 |= 1 << irq_num;
    //         }

    //         self.lpi.ctlr.wait_pending_store();
    //     }

    //     fn pend_interrupt(&mut self, irq_num: usize, pend: bool) {
    //         if pend {
    //             self.sgi_ppi.icpendr0 &= !(1 << irq_num);
    //             self.sgi_ppi.ispendr0 |= 1 << irq_num;
    //         } else {
    //             self.sgi_ppi.ispendr0 &= !(1 << irq_num);
    //             self.sgi_ppi.icpendr0 |= 1 << irq_num;
    //         }

    //         self.lpi.ctlr.wait_pending_store();
    //     }

    //     #[must_use]
    //     pub fn enable_sgi(&mut self, irq_num: usize, enable: bool) -> bool {
    //         if !(0..16).contains(&irq_num) {
    //             return false;
    //         }

    //         self.enable_interrupt(irq_num, enable);
    //         true
    //     }

    //     #[must_use]
    //     pub fn enable_ppi(&mut self, irq_num: usize, enable: bool) -> bool {
    //         if !(16..32).contains(&irq_num) {
    //             return false;
    //         }

    //         self.enable_interrupt(irq_num, enable);
    //         true
    //     }

    //     #[must_use]
    //     pub fn pend_sgi(&mut self, irq_num: usize, pend: bool) -> bool {
    //         if !(0..16).contains(&irq_num) {
    //             return false;
    //         }

    //         self.pend_interrupt(irq_num, pend);
    //         true
    //     }

    //     #[must_use]
    //     pub fn pend_ppi(&mut self, irq_num: usize, pend: bool) -> bool {
    //         if !(16..32).contains(&irq_num) {
    //             return false;
    //         }

    //         self.pend_interrupt(irq_num, pend);
    //         true
    //     }

    /// Initialize the control interface to the CPU
    /// through the ICC_* system registers.
    pub fn init_icc(&mut self) {}

    /// Get the GIC version.
    pub fn version(&self) -> GicVersion {
        self.version
    }

    /// Get the maximum SPI line.
    pub fn max_spi_id(&self) -> usize {
        self.max_spi
    }

    /// Number of CPUs
    pub fn num_cpus(&self) -> usize {
        self.num_cpus
    }
}
