#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupts;
mod vga_driver;

use core::panic::PanicInfo;
use multiboot2::BootInformation;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
extern "C" fn kernel_main(multiboot_information_address: usize) {
    interrupts::init_interrupts();

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

    let elf_sections_tag = boot_info.elf_sections().expect("Elf-sections tag required");

    println!("kernel sections:");
    for section in elf_sections_tag {
        println!(
            "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(),
            section.size(),
            section.flags()
        );
    }
}
