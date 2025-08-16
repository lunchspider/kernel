#![no_std]
#![no_main]
#![feature(abi_x86_interrupt, ptr_internals)]

mod gdt;
mod interrupts;
mod memory;
mod pic;
mod vga_driver;

use core::panic::PanicInfo;
use memory::{FrameAllocator, RecursivePageTable};
use multiboot2::BootInformation;
use x86_64::{instructions::hlt, registers::control::Efer, PhysAddr, VirtAddr};

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

    for i in 0.. {
        let frame = frame_allocator.allocate_frame();
        if let None = frame {
            println!("allocated {} frames", i);
            break;
        }
    }

    let pages = RecursivePageTable::new(VirtAddr::new(0xffffffff_fffff000));

    let res = unsafe { pages.translate(VirtAddr::new(0xb8000)) };
    println!("got frame: {:?}", res);

    //println!("kernel_start: {:?}", kernel_start);

    //let res = unsafe { pages.translate(VirtAddr::new(kernel_start)) };

    //println!("got frame: {:?}", res);
    //
    enable_nxe_bit();
    enable_write_protect_bit();

    hlt_loop();
}

fn enable_nxe_bit() {
    let nxe_bit = 1 << 11;
    unsafe {
        let efer = Efer::read_raw();
        Efer::write_raw(efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control::{Cr0, Cr0Flags};

    unsafe { Cr0::write(Cr0Flags::WRITE_PROTECT | Cr0::read()) };
}
