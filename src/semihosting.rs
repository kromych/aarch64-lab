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

pub struct Semihosting;

impl Semihosting {
    pub fn exit(&self, code: u64) -> ! {
        const SYS_EXIT: u32 = 0x18;
        const APPLICATION_EXIT: u64 = 0x20026;

        let data = [APPLICATION_EXIT, code];
        unsafe {
            semi_call(SYS_EXIT, data.as_ptr());
        }
        unsafe { unreachable_unchecked() }
    }

    pub fn write_char(&self, c: char) {
        const SYS_WRITEC: u32 = 0x03;
        let data = [c as u64];
        unsafe {
            semi_call(SYS_WRITEC, data.as_ptr());
        }
    }

    pub fn write_str0(&self, s: &[u8]) {
        const SYS_WRITE0: u32 = 0x04;
        unsafe {
            semi_call(SYS_WRITE0, s.as_ptr().cast());
        }
    }

    pub fn write_hex(&self, h: u64) {
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
        self.write_str0(hs);
    }
}

impl core::fmt::Write for Semihosting {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let buf = core::mem::MaybeUninit::<[u8; 512]>::uninit();

        let bytes = s.as_bytes();
        let mut buf = unsafe { buf.assume_init() };

        let mut printed = 0;
        while printed < s.len() {
            let available = core::cmp::min(buf.len() - 2, s.len() - printed);
            buf[0..available].copy_from_slice(&bytes[..available]);
            buf[available] = 0;

            self.write_str0(&buf);

            printed += available;
        }

        Ok(())
    }
}
