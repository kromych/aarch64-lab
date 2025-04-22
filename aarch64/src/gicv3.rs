//! Lean and mean GICv3 interface.
//!
//! GICv3 has two main components:
//! 1. GICD (distributor) - the central hub for all interrupts
//! 2. GICR (re-distributors) - per-CPU interrupt management
//!
//! GICD functions, most notably:
//!
//! - routes interrupts to correct CPU,
//! - stores global interrupt state (enabled/pending),
//! - handles interrupt prioritization,
//! - broadcasts SGIs (Software Generated Interrupts)
//!
//! Crucial GICR functions (per CPU):
//!
//! - manages CPU-private interrupts (SGIs 0-15, PPIs 16-31),
//! - handles interrupt signaling to individual cores,
//! - provides wakeup control for power management
//!

// Some GICD Registers.

use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

/// Global control (enable/disable)
const GICD_CTLR: usize = 0x0000;
/// Interrupt Set-Enable (group interrupts in 32-bit banks)
const GICD_ISENABLER_N: usize = 0x0100;
/// Clear Pending status
const GICD_ICPENDR_N: usize = 0x0280;
/// Priority configuration (each int has 8-bit priority)
const GICD_IPRIORITYR_N: usize = 0x0400;
/// Software Generated Interrupt trigger
const GICD_SGIR: usize = 0x0F00;

// Some GICR registers

/// Local CPU interface control
const GICR_CTLR: usize = 0x0000;
/// Power management (sleep/wake)
const GICR_WAKER: usize = 0x0014;
///
const GICR_IGROUPR0: usize = 0x0080;
/// Enable local interrupts (SGIs/PPIs)
const GICR_ISENABLER0: usize = 0x0100;
///
const GICR_IPRIORITYR_N: usize = 0x0400;
///
const GICR_ICFGR0: usize = 0x0C00;
///
const GICR_ICFGR1: usize = 0x0C04;

pub struct Gicv3<'a> {
    gicd_base: &'a mut [AtomicU32],
    gicr_base: &'a mut [AtomicU32],
}

impl<'a> Gicv3<'a> {
    pub fn new(gicd_base: &'a mut [AtomicU32], gicr_base: &'a mut [AtomicU32]) -> Self {
        Gicv3 {
            gicd_base,
            gicr_base,
        }
    }

    pub fn init(&mut self) {
        // Disable distributor (central control)
        self.gicd_base[GICD_CTLR / 4].store(0, Ordering::SeqCst);
        // Wait for disable
        while (self.gicd_base[GICD_CTLR / 4].load(Ordering::Acquire) & 1) != 0 {}

        // Set all interrupts to group 1 (non-secure)
        for i in 0..32 {
            self.gicr_base[GICR_IGROUPR0 / 4 + i].store(0xFFFFFFFF, Ordering::Release);
        }

        // Set all interrupts to level-sensitive
        self.gicr_base[GICR_ICFGR0 / 4].store(0, Ordering::Release);
        self.gicr_base[GICR_ICFGR1 / 4].store(0, Ordering::Release);

        // Set medium priority interrupts
        for i in 0..32 {
            self.gicr_base[GICR_IPRIORITYR_N / 4 + i].store(0x80, Ordering::Release);
        }

        // Wake redistributor

        let waker = self.gicr_base[GICR_WAKER / 4].load(Ordering::Acquire);
        self.gicr_base[GICR_WAKER / 4].store(waker & !(1 << 1), Ordering::Release); // Clear ProcessorSleep
        while (self.gicr_base[GICR_WAKER / 4].load(Ordering::Acquire) & (1 << 2)) != 0 {} // Wait for ChildrenAsleep=0

        // Enable distributor (global interrupts)
        self.gicd_base[GICD_CTLR / 4].store(1, Ordering::Release);

        // Enable redistributor (local interrupts)
        self.gicr_base[GICR_CTLR / 4].store(1, Ordering::Release);
    }

    pub fn send_sgi(&mut self, sgi_id: u32, cpu_mask: u32) {
        self.gicd_base[GICD_SGIR / 4].store((cpu_mask << 16) | (sgi_id & 0xF), Ordering::Release);
    }

    pub fn enable_interrupt(&mut self, int_id: u32) {
        if int_id < 32 {
            // SGIs and PPIs - use redistributor
            let mask = self.gicr_base[GICR_ISENABLER0 / 4].load(Ordering::Acquire);
            self.gicr_base[GICR_ISENABLER0 / 4].store(mask | (1 << int_id), Ordering::Release);
        } else {
            // SPIs - use distributor
            let mask = self.gicd_base[GICD_ISENABLER_N / 4 + (int_id / 32) as usize]
                .load(Ordering::Acquire);
            self.gicd_base[GICD_ISENABLER_N / 4 + (int_id / 32) as usize]
                .store(mask | (1 << (int_id % 32)), Ordering::Release);
        }
    }

    pub fn disable_interrupt(&mut self, int_id: u32) {
        if int_id < 32 {
            // SGIs and PPIs - use redistributor
            let mask = self.gicr_base[GICR_ISENABLER0 / 4].load(Ordering::Acquire);
            self.gicr_base[GICR_ISENABLER0 / 4].store(mask & !(1 << int_id), Ordering::Release);
        } else {
            // SPIs - use distributor
            let mask = self.gicd_base[GICD_ISENABLER_N / 4 + (int_id / 32) as usize]
                .load(Ordering::Acquire);
            self.gicd_base[GICD_ISENABLER_N / 4 + (int_id / 32) as usize]
                .store(mask & !(1 << (int_id % 32)), Ordering::Release);
        }
    }

    pub fn enable_local_interrupts(&mut self) {
        self.gicr_base[GICR_ISENABLER0 / 4].store(1, Ordering::Release);
    }

    pub fn disable_local_interrupts(&mut self) {
        self.gicr_base[GICR_ISENABLER0 / 4].store(0, Ordering::Release);
    }
}
