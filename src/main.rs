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
    let mut semi: semihosting::Semihosting = semihosting::Semihosting;
    let id = pl011.reset_and_init();

    semi.write_char('H');
    pl011.write_str("ello ").ok();
    semi.write_hex(id);
    semi.write_char('\n');

    writeln!(semi, "Semihosting {id:#x}").ok();
    writeln!(pl011, "PL011 {id:#x}").ok();

    semi.exit(0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let semi: semihosting::Semihosting = semihosting::Semihosting;

    semi.write_str0(b"panic\n\0");
    semi.exit(!0);
}
