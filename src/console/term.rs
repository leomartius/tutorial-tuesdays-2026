use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{self, Color as TermColor, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{
        self, BeginSynchronizedUpdate, ClearType::All, EndSynchronizedUpdate, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    },
};

use super::{Buffer, Color as ApiColor, ConsoleError, Event, Key};

pub struct Terminal {
    screen: AltScreen,
}

impl Terminal {
    pub fn new(min_width: usize, min_height: usize) -> Result<Self, ConsoleError> {
        debug_assert!(min_width <= u16::MAX.into() && min_height <= u16::MAX.into());
        Self::check_terminal_size(min_width as u16, min_height as u16)?;
        let mut screen = AltScreen::enter(io::stdout())?;
        execute!(screen.stdout, style::ResetColor, terminal::Clear(All))?;
        Ok(Self { screen })
    }

    fn check_terminal_size(min_width: u16, min_height: u16) -> Result<(), ConsoleError> {
        let (width, height) = terminal::size()?;
        if width < min_width || height < min_height {
            return Err(ConsoleError::TermSize {
                required: (min_width, min_height),
                actual: (width, height),
            });
        }
        Ok(())
    }

    fn stdout(&self) -> &Stdout {
        &self.screen.stdout
    }

    pub fn display(
        &mut self,
        current: &Buffer,
        previous: &Buffer,
        cursor: Option<(usize, usize)>,
    ) -> Result<(), io::Error> {
        debug_assert!(current.width == previous.width && current.height == previous.height);
        let (mut cx, mut cy) = (usize::MAX, usize::MAX);
        let mut last_fg: Option<ApiColor> = None;
        let mut last_bg: Option<ApiColor> = None;
        queue!(self.stdout(), BeginSynchronizedUpdate)?;
        queue!(self.stdout(), cursor::Hide)?;
        for y in 0..current.height {
            for x in 0..current.width {
                let curr = current.get(x, y);
                let prev = previous.get(x, y);
                if curr != prev {
                    if (x != cx) || (y != cy) {
                        queue!(self.stdout(), MoveTo(x as u16, y as u16))?;
                    }
                    if last_fg != Some(curr.fg) {
                        queue!(self.stdout(), SetForegroundColor(convert_color(curr.fg)))?;
                        last_fg = Some(curr.fg);
                    }
                    if last_bg != Some(curr.bg) {
                        queue!(self.stdout(), SetBackgroundColor(convert_color(curr.bg)))?;
                        last_bg = Some(curr.bg);
                    }
                    queue!(self.stdout(), Print(curr.ch))?;
                    (cx, cy) = (x + 1, y);
                }
            }
        }
        if let Some((x, y)) = cursor {
            debug_assert!(x < current.width && y < current.height);
            queue!(self.stdout(), MoveTo(x as u16, y as u16), cursor::Show)?;
        }
        queue!(self.stdout(), EndSynchronizedUpdate)?;
        self.stdout().flush()
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), io::Error> {
        execute!(self.stdout(), SetTitle(title))
    }

    pub fn alert(&mut self) -> Result<(), io::Error> {
        execute!(self.stdout(), Print('\x07'))
    }

    pub fn reset(&mut self) -> Result<(), io::Error> {
        execute!(self.stdout(), style::ResetColor, terminal::Clear(All))
    }

    pub fn read_event(&self) -> Result<Event, io::Error> {
        loop {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    return Ok(Event::Abort);
                }
                if key.code == KeyCode::Char('l') && key.modifiers == KeyModifiers::CONTROL {
                    return Ok(Event::ClearScreen);
                }
                if key.modifiers == KeyModifiers::NONE {
                    let e = match key.code {
                        KeyCode::Char(ch) => Event::KeyChar(ch),
                        KeyCode::Left => Event::KeySpecial(Key::Left),
                        KeyCode::Right => Event::KeySpecial(Key::Right),
                        KeyCode::Up => Event::KeySpecial(Key::Up),
                        KeyCode::Down => Event::KeySpecial(Key::Down),
                        KeyCode::Home => Event::KeySpecial(Key::Home),
                        KeyCode::End => Event::KeySpecial(Key::End),
                        KeyCode::PageUp => Event::KeySpecial(Key::PgUp),
                        KeyCode::PageDown => Event::KeySpecial(Key::PgDn),
                        _ => continue,
                    };
                    return Ok(e);
                }
            }
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = execute!(
            self.stdout(),
            style::ResetColor,
            terminal::Clear(All),
            MoveTo(0, 0)
        );
    }
}

fn convert_color(color: ApiColor) -> TermColor {
    match color {
        ApiColor::Default => TermColor::Reset,
        ApiColor::Black => TermColor::Black,
        ApiColor::Red => TermColor::DarkRed,
        ApiColor::Green => TermColor::DarkGreen,
        ApiColor::Yellow => TermColor::DarkYellow,
        ApiColor::Blue => TermColor::DarkBlue,
        ApiColor::Magenta => TermColor::DarkMagenta,
        ApiColor::Cyan => TermColor::DarkCyan,
        ApiColor::White => TermColor::Grey,
        ApiColor::BrightBlack => TermColor::DarkGrey,
        ApiColor::BrightRed => TermColor::Red,
        ApiColor::BrightGreen => TermColor::Green,
        ApiColor::BrightYellow => TermColor::Yellow,
        ApiColor::BrightBlue => TermColor::Blue,
        ApiColor::BrightMagenta => TermColor::Magenta,
        ApiColor::BrightCyan => TermColor::Cyan,
        ApiColor::BrightWhite => TermColor::White,
        ApiColor::Ansi8(color) => {
            debug_assert!(
                color > 15,
                "invalid ANSI color {}; values 0-15 are named colors",
                color
            );
            TermColor::AnsiValue(color)
        }
        ApiColor::Rgb24(r, g, b) => TermColor::Rgb { r, g, b },
    }
}

struct AltScreen {
    stdout: Stdout,
}

impl AltScreen {
    pub fn enter(mut stdout: Stdout) -> Result<Self, io::Error> {
        match Self::try_enter(&mut stdout) {
            Ok(_) => Ok(Self { stdout }),
            Err(error) => {
                Self::leave(&mut stdout);
                Err(error)
            }
        }
    }

    fn try_enter(stdout: &mut Stdout) -> Result<(), io::Error> {
        terminal::enable_raw_mode()?;
        queue!(stdout, EnterAlternateScreen)?;
        queue!(stdout, cursor::Hide)?;
        stdout.flush()
    }

    fn leave(stdout: &mut Stdout) {
        let _ = execute!(stdout, cursor::Show);
        let _ = execute!(stdout, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        Self::leave(&mut self.stdout);
    }
}
