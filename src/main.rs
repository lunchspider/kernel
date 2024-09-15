#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_x: &PanicInfo<'_>) -> ! {
    loop {}
}

core::arch::global_asm!(include_str!("boot.s"));

#[allow(unused)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta = 13,
    LightBrown = 14,
    White = 15,
}

const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

struct Terminal {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: *mut u16,
}

fn vga_entry(ch: char, color: u8) -> u16 {
    (ch as u8 as u16) | (color as u16) << 8
}

fn vga_entry_color(fg: VGAColor, bg: VGAColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}

impl Terminal {
    unsafe fn new() -> Self {
        let terminal_color = vga_entry_color(VGAColor::LightGrey, VGAColor::Black);
        let terminal_buffer: *mut u16 = 0xB8000 as _;
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_HEIGHT {
                let index = (y * VGA_WIDTH + x) as isize;
                *terminal_buffer.offset(index) = vga_entry(' ', terminal_color);
            }
        }
        Self {
            terminal_row: 0,
            terminal_column: 0,
            terminal_buffer,
            terminal_color,
        }
    }

    fn put_char(&mut self, ch: char) {
        let index = (self.terminal_row * VGA_WIDTH + self.terminal_column) as isize;
        unsafe {
            *self.terminal_buffer.offset(index) = vga_entry(ch, self.terminal_color);
        }
    }

    fn print_str(&mut self, s: &'static str) {
        for ch in s.chars() {
            self.put_char(ch);
        }
    }
}

#[no_mangle]
extern "C" fn kernel_main() {
    unsafe {
        let mut term = Terminal::new();
        term.print_str("Hello, Kernel!");
        loop {}
    }
}
