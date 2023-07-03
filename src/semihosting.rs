//! Semihosting for Aarch64
//!
//! See [Reference](https://github.com/ARM-software/abi-aa/blob/main/semihosting/semihosting.rst)
//! for futher details.

use core::arch::asm;
use core::hint::unreachable_unchecked;

macro_rules! host_trap {
    () => {
        "hlt #0xF000"
    };
}

#[inline]
unsafe fn semi_call(number: u32, mut _parameter: *const u64) -> u64 {
    let r;
    unsafe {
        asm!(
            host_trap!(),
            in("w0") number,
            inout("x1") _parameter,
            lateout("x0") r,
            options(nostack, preserves_flags),
        );
    }
    r
}

pub fn exit(code: u64) -> ! {
    const SYS_EXIT: u32 = 0x18;
    const APPLICATION_EXIT: u64 = 0x20026;

    let data = [APPLICATION_EXIT, code];
    unsafe {
        semi_call(SYS_EXIT, data.as_ptr());
    }
    unsafe { unreachable_unchecked() }
}

pub fn write_char(c: char) {
    const SYS_WRITEC: u32 = 0x03;
    let data = [c as u64];
    unsafe {
        semi_call(SYS_WRITEC, data.as_ptr());
    }
}

pub fn write_str0(s: &[u8]) {
    const SYS_WRITE0: u32 = 0x04;
    unsafe {
        semi_call(SYS_WRITE0, s.as_ptr().cast());
    }
}

pub fn write_hex(h: u64) {
    let mut hs = [0_u16; 11];
    hs[0] = u16::from_le_bytes([b'0', b'x']);

    let hexn = |nibble| match nibble {
        0..=9 => nibble + b'0',
        10..=15 => nibble - 10 + b'a',
        _ => panic!("Nibble out of range"),
    };
    for (n, &b) in h.to_be_bytes().iter().enumerate() {
        hs[n + 1] = ((hexn(b & 0xf) as u16) << 8) | (hexn(b >> 4) as u16);
    }
    let hs = unsafe { core::slice::from_raw_parts(hs.as_ptr() as *const u8, hs.len() * 2) };
    write_str0(hs);
}
