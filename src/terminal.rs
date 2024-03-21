use crate::Position;
use std::io::{self, stdout, Error, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, color, cursor};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _raw_stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    /// # Errors
    /// Returns an error if the terminal size can't be obtained or if the terminal can't be put into raw mode.
    pub fn new() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2 /* status bar & message bar */),
            },
            // As long as this variable is alive, we are in raw mode.
            // For information on what are terminal modes, see
            // https://www.gnu.org/software/mit-scheme/documentation/stable/mit-scheme-ref/Terminal-Mode.html.
            _raw_stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen() {
        print!("{}", clear::All);
    }

    pub fn clear_current_line() {
        print!("{}", clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        print!("{}", color::Bg(color));
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_bg_color() {
        print!("{}", color::Bg(color::Reset));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    /// The position is 0-based.
    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!("{}", cursor::Goto(x as u16, y as u16));
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    /// # Errors
    /// Returns an error if the terminal is not flushed successfully.
    pub fn flush() -> Result<(), Error> {
        io::stdout().flush()
    }

    /// # Errors
    /// Returns an error if the key can't be read from the terminal.
    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }
}
