#![no_std]
#![no_main]

use core::fmt::Write;

core::arch::global_asm!(include_str!("start.S"));

mod pl011;
mod reloc;
mod semihosting;

#[no_mangle]
fn main() -> ! {
    let mut pl011: pl011::Pl011 = pl011::Pl011;
    let id = pl011.reset_and_init();

    semihosting::write_char('H');
    pl011.write_str("ello ").ok();
    semihosting::write_hex(id);

    // Rust compiler produces globals for formatting calls,
    // need to relocate.
    //
    // write!(pl011, "PL011 {id:x}").ok();

    semihosting::exit(0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    semihosting::write_str0(b"panic\n\0");
    semihosting::exit(!0);
}
