#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(bg: Color, fg: Color) -> ColorCode {
        ColorCode((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const MAX_WIDTH: usize = 80;
const MAX_HEIGHT: usize = 25;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; MAX_WIDTH]; MAX_HEIGHT],
}

struct Writer {
    column: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= MAX_WIDTH {
                    self.new_line();
                }

                let row = MAX_HEIGHT - 1;
                let col = self.column;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };

                self.column += 1;
            }
        }
    }

    fn write_text(&mut self, x: &str) {
        let bytes = x.as_bytes();

        for byte in bytes.iter() {
            self.write_byte(*byte);
        }
    }

    fn new_line(&mut self) {
        for row in 1..MAX_HEIGHT {
            for col in 0..MAX_WIDTH {
                let ch = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = ch;
            }
        }

        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..MAX_WIDTH {
            self.buffer.chars[MAX_HEIGHT - 1][col] = blank;
        }

        self.column = 0;
    }
}

pub fn print(x: &str) {
    let mut writer = Writer {
        column: 2,
        color_code: ColorCode::new(Color::DarkGray, Color::Red),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_text(x);
}
