#![no_std]
#![no_main]

mod vga_driver;

use core::panic::PanicInfo;
use multiboot2::BootInformation;

#[panic_handler]
fn panic(_x: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn kernel_main(multiboot_information_address: usize) {
    let boot_info = unsafe {
        BootInformation::load(multiboot_information_address as *const _)
            .expect("Cannot load boot info")
    };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas: ");
    for area in memory_map_tag.memory_areas() {
        println!(
            "    start: 0x{:x}, length: 0x{:x}",
            area.start_address(),
            area.size()
        );
    }
}
