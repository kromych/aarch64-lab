#![no_std]
#![no_main]

use core::fmt::Write;

core::arch::global_asm!(include_str!("start.S"));

mod pl011;
mod semihosting;

#[no_mangle]
fn main() -> ! {
    let mut pl011: pl011::Pl011 = pl011::Pl011;
    let _id = pl011.reset_and_init();

    semihosting::write_char('H');
    pl011.write_str("ello").ok();
    //write!(pl011, "PL011 {_id:x}").ok();

    semihosting::exit(0);
}

#[no_mangle]
fn relocate() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    semihosting::exit(!0);
}
