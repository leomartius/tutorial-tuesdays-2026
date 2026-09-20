mod term;

use std::{io, mem};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error(
        "Terminal window too small. Please resize to at least {}x{} characters.",
        .required.0, .required.1
    )]
    TermSize {
        required: (u16, u16),
        actual: (u16, u16),
    },
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub struct Console {
    terminal: term::Terminal,
    front: Buffer,
    back: Buffer,
    cursor: Option<(usize, usize)>,
}

impl Console {
    pub fn new(width: i32, height: i32, title: &str) -> Result<Self, ConsoleError> {
        debug_assert!(width >= 0 && height >= 0);
        let (width, height) = (width as usize, height as usize);
        let mut terminal = term::Terminal::new(width, height)?;
        terminal.set_title(title)?;
        Ok(Self {
            terminal,
            front: Buffer::new(width, height),
            back: Buffer::new(width, height),
            cursor: None,
        })
    }

    pub fn clear(&mut self) {
        self.back.clear();
        self.cursor = None;
    }

    pub fn clear_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        debug_assert!(x >= 0 && y >= 0 && width >= 0 && height >= 0);
        let (x, y, width, height) = (x as usize, y as usize, width as usize, height as usize);
        self.back.fill_rect(x, y, width, height, Cell::default());
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, bg: Color) {
        debug_assert!(x >= 0 && y >= 0 && width >= 0 && height >= 0);
        let (x, y, width, height) = (x as usize, y as usize, width as usize, height as usize);
        let fill = Cell {
            bg,
            ..Default::default()
        };
        self.back.fill_rect(x, y, width, height, fill);
    }

    fn set(&mut self, x: usize, y: usize, ch: char, fg: Color, bg: Color) {
        self.back.set(x, y, Cell::new(ch, fg, bg));
    }

    pub fn set_cell(&mut self, x: i32, y: i32, ch: char, fg: Color, bg: Color) {
        debug_assert!(x >= 0 && y >= 0);
        self.set(x as usize, y as usize, ch, fg, bg);
    }

    pub fn print_char(&mut self, x: i32, y: i32, ch: char, fg: Color) {
        debug_assert!(x >= 0 && y >= 0);
        let old = self.back.get(x as usize, y as usize);
        self.set(x as usize, y as usize, ch, fg, old.bg);
    }

    pub fn print_str(&mut self, x: i32, y: i32, text: &str, fg: Color) {
        debug_assert!(x >= 0 && y >= 0);
        let (x0, y) = (x as usize, y as usize);
        for (dx, ch) in text.chars().enumerate() {
            if x0 + dx >= self.back.width {
                break;
            }
            let old = self.back.get(x0 + dx, y);
            self.set(x0 + dx, y, ch, fg, old.bg);
        }
    }

    pub fn dim(&mut self) {
        self.back
            .apply(|cell| *cell = Cell::new(cell.ch, cell.fg.to_dim(), cell.bg.to_black()));
    }

    pub fn show_cursor(&mut self, x: i32, y: i32) {
        debug_assert!(x >= 0 && y >= 0);
        self.cursor = Some((x as usize, y as usize));
    }

    pub fn hide_cursor(&mut self) {
        self.cursor = None;
    }

    pub fn display(&mut self) -> Result<(), ConsoleError> {
        mem::swap(&mut self.back, &mut self.front);
        self.terminal
            .display(&self.front, &self.back, self.cursor)?;
        Ok(())
    }

    pub fn alert(&mut self) -> Result<(), ConsoleError> {
        self.terminal.alert()?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), ConsoleError> {
        self.front.clear();
        self.terminal.reset()?;
        Ok(())
    }

    pub fn read_event(&self) -> Result<Event, ConsoleError> {
        Ok(self.terminal.read_event()?)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    // default foreground/background
    Default,
    // terminal palette: basic colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // terminal palette: bright colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    // ANSI 8-bit colors
    Ansi8(u8),
    // true color (24-bit)
    Rgb24(u8, u8, u8),
}

impl Color {
    // 6x6x6 color cube (216 colors)
    pub const fn ansi_cube(red: u8, green: u8, blue: u8) -> Self {
        debug_assert!(red < 6 && green < 6 && blue < 6);
        Color::Ansi8(16 + (red * 36) + (green * 6) + blue)
    }

    // 1-24 grayscale (24 levels)
    pub const fn ansi_grayscale(gray: u8) -> Self {
        debug_assert!(gray <= 25);
        match gray {
            0 => Color::ansi_cube(0, 0, 0),
            25 => Color::ansi_cube(5, 5, 5),
            _ => Color::Ansi8(231 + gray),
        }
    }

    // 24-bit RGB
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Color::Rgb24(red, green, blue)
    }

    fn to_dim(self) -> Self {
        match self {
            // light/dark pairs
            Color::BrightRed => Color::Red,
            Color::BrightGreen => Color::Green,
            Color::BrightYellow => Color::Yellow,
            Color::BrightBlue => Color::Blue,
            Color::BrightMagenta => Color::Magenta,
            Color::BrightCyan => Color::Cyan,
            // grayscale
            Color::White => Color::BrightBlack,
            Color::BrightWhite => Color::BrightBlack,
            // unchanged
            _ => self,
        }
    }

    fn to_black(self) -> Self {
        match self {
            Color::Default => self,
            _ => Color::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
}

impl Cell {
    fn new(ch: char, fg: Color, bg: Color) -> Self {
        Self { ch, fg, bg }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: '\x20', // ASCII space
            fg: Color::Default,
            bg: Color::Default,
        }
    }
}

struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
        }
    }

    fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        debug_assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        debug_assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x] = cell;
    }

    fn fill_rect(&mut self, x0: usize, y0: usize, width: usize, height: usize, value: Cell) {
        debug_assert!(x0 + width <= self.width && y0 + height <= self.height);
        for y in y0..y0 + height {
            self.cells[y * self.width + x0..y * self.width + x0 + width].fill(value);
        }
    }

    fn apply<F>(&mut self, transform: F)
    where
        F: FnMut(&mut Cell),
    {
        self.cells.iter_mut().for_each(transform);
    }
}

pub enum Event {
    Abort,
    ClearScreen,
    KeyChar(char),
    KeySpecial(Key),
}

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PgUp,
    PgDn,
}
