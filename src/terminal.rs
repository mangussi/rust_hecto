use std::io::{self, stdout, Write};
use termion::{
    color,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::Position;

// const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    /// # Errors
    ///
    /// May return an `Err`
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        #[allow(clippy::cast_possible_truncation)]
        let x = x.saturating_add(1) as u16;
        #[allow(clippy::cast_possible_truncation)]
        let y = y.saturating_add(1) as u16;
        // let y = u16::try_from(y.saturating_add(1));
        print!("{}", termion::cursor::Goto(x, y));
    }

    /// # Errors
    ///
    /// It is considered an error if not all bytes could be
    /// written due to I/O errors or EOF being reached
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn set_bg_color(color: color::Rgb) {
        let _ = color;
        print!("{}", termion::style::Invert);
    }

    pub fn reset_bg_color() {
        // print!("{}", color::Bg(color::Reset));
        print!("{}", termion::style::Reset);
    }

    pub fn set_fg_color(color: color::Rgb) {
        print!("{}", color::Fg(color));
    }

    pub fn reset_fg_color() {
        print!("{}", color::Fg(color::Reset));
    }

    /// # Errors
    ///
    /// Will return `Err` if an issue is found
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}
