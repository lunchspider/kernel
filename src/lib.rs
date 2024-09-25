#![no_std]
#![no_main]

mod vga_driver;

use core::{fmt::Write, panic::PanicInfo};

use vga_driver::VGATerminal;

#[panic_handler]
fn panic(_x: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn kernel_main() {
    unsafe {
        let mut term = VGATerminal::new();
        for _ in 0..25 {
            term.write_str("Hello, Kernel!\n");
            term.write_str("Hello, New Line!!!\n");
        }
        loop {}
    }
}
