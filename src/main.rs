#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("start.S"));

#[no_mangle]
fn main() -> ! {
    loop {
        unsafe {
            core::arch::asm!("yield");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
