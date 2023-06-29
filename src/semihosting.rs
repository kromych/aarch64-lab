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

// pub fn write_str(s: &str) {
//     const SYS_WRITE0: u32 = 0x04;
// }

pub fn write_char(c: char) {
    const SYS_WRITEC: u32 = 0x03;
    let data = [c as u64];
    unsafe {
        semi_call(SYS_WRITEC, data.as_ptr());
    }
}
