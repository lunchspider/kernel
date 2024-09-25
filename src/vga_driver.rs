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

pub struct VGATerminal<'a> {
    terminal_row: usize,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: &'a mut Buffer,
}

pub struct Buffer([[Volatile<VGAChar>; VGA_WIDTH]; VGA_HEIGHT]);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VGAChar {
    pub ascii: u8,
    pub color: u8,
}

impl VGAChar {
    fn new(ch: char, color: u8) -> Self {
        Self {
            ascii: ch as u8,
            color,
        }
    }
}

fn vga_entry_color(fg: VGAColor, bg: VGAColor) -> u8 {
    fg as u8 | (bg as u8) << 4
}

impl VGATerminal<'_> {
    pub unsafe fn new() -> Self {
        let terminal_color = vga_entry_color(VGAColor::LightGrey, VGAColor::Black);
        let mut terminal_buffer = &mut *(0xb8000 as *mut Buffer);
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_HEIGHT {
                terminal_buffer.0[y][x].write(VGAChar::new(' ', terminal_color));
            }
        }
        Self {
            terminal_row: VGA_HEIGHT - 1,
            terminal_column: 0,
            terminal_buffer,
            terminal_color,
        }
    }

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

    pub fn print_str(&mut self, s: &'static str) {
        for ch in s.chars() {
            if ch != '\n' {
                self.put_char(ch);
            }
            self.increment_column();
            if ch == '\n' {
                self.new_line();
                self.terminal_column = 0;
            }
        }
    }
}
