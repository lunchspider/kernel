use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(unused)]
#[repr(C)]
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

pub struct VGATerminal {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: &'static mut Buffer,
}

pub struct Buffer([[Volatile<VGAChar>; VGA_WIDTH]; VGA_HEIGHT]);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VGAChar {
    pub ascii: u8,
    pub color: u8,
}

lazy_static! {
    pub static ref TERMINAL: Mutex<VGATerminal> = Mutex::new(VGATerminal {
        terminal_row: VGA_HEIGHT - 1,
        terminal_column: 0,
        terminal_color: vga_entry_color(VGAColor::LightGrey, VGAColor::Black),
        terminal_buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl VGAChar {
    fn new(ch: char, color: u8) -> Self {
        Self {
            ascii: ch as u8,
            color,
        }
    }
}

const fn vga_entry_color(fg: VGAColor, bg: VGAColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}

impl VGATerminal {
    pub fn put_char(&mut self, ch: char) {
        self.terminal_buffer.0[self.terminal_row][self.terminal_column]
            .write(VGAChar::new(ch, self.terminal_color));
    }

    pub fn increment_column(&mut self) {
        if self.terminal_column == VGA_WIDTH - 1 {
            self.terminal_column = 0;
        } else {
            self.terminal_column += 1;
        }
    }

    pub fn new_line(&mut self) {
        // shifting all the rows one up
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let character = self.terminal_buffer.0[row][col].read();
                self.terminal_buffer.0[row - 1][col].write(character);
            }
        }
        self.terminal_column = 0;
        self.clear_row(VGA_HEIGHT - 1);
    }

    pub fn clear_row(&mut self, row: usize) {
        let character = VGAChar::new(' ', self.terminal_color);
        for col in 0..VGA_WIDTH {
            self.terminal_buffer.0[row][col].write(character);
        }
    }
}

impl core::fmt::Write for VGATerminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            if !ch.is_ascii() {
                return Err(core::fmt::Error);
            }
            if ch != '\n' {
                self.put_char(ch);
            }
            self.increment_column();
            if ch == '\n' {
                self.new_line();
                self.terminal_column = 0;
            }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_driver::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    TERMINAL.lock().write_fmt(args).unwrap();
}
