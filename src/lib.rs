#![no_std]
#![no_main]
#![feature(abi_x86_interrupt, ptr_internals)]

mod gdt;
mod interrupts;
mod memory;
mod pic;
mod vga_driver;

use core::panic::PanicInfo;
use multiboot2::BootInformation;
use x86_64::{instructions::hlt, VirtAddr};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

#[no_mangle]
extern "C" fn kernel_main(multiboot_information_address: usize) {
    gdt::init();
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

    // 0 because i didn't do any offset
    let phys_mem_offset = VirtAddr::new(0);

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        //0x201008,
        // some stack page
        //0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        0,
    ];

    let kernel_start = boot_info
        .elf_sections()
        .unwrap()
        .map(|s| s.start_address())
        .min()
        .unwrap();
    let kernel_end = boot_info
        .elf_sections()
        .unwrap()
        .map(|s| s.start_address() + s.size())
        .min()
        .unwrap();
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size() as usize);

    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        multiboot_start,
        multiboot_end,
        memory_map_tag.memory_areas(),
    );

    hlt_loop();
}
