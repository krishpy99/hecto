use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
            // top-left corner
            cursor_position: Position { x: 0, y: 0 },
        }
    }
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            // NOTE: The screen is refreshed before quitting.
            if let Err(e) = &self.refresh_screen() {
                die(e);
            }
            if self.should_quit {
                break;
            }
            if let Err(e) = &self.process_keypress() {
                die(e);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide(); // prevent the cursor from blinking
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.should_quit {
            Terminal::clear_screen();
            Editor::farewell();
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn farewell() {
        println!("Goodbye.\r");
    }

    /// Draws a tilde in each row, meaning that row is not part of the file and can't contain any text
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        // The last line is kept empty for the status bar.
        for row in 0..height - 1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_msg = format!("Hecto editor -- version {VERSION}");
        let term_width = self.terminal.size().width as usize;
        let msg_len = welcome_msg.len();
        // The padding is the number of spaces to add to the left of the message.
        let padding = term_width.saturating_sub(msg_len) / 2;
        let spaces = " ".repeat(padding.saturating_add(1 /* for ~ */));
        welcome_msg = format!("~{spaces}{welcome_msg}\r");
        welcome_msg.truncate(term_width);
        println!("{welcome_msg}\r");
    }

    /// Where the handling logics go.
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Getting a `quit` signal isn't an error.
            Key::Ctrl('q') => {
                self.should_quit = true;
                Ok(())
            }
            Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(pressed_key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;

        // Check if the cursor has moved outside of the visible window,
        // and if so, adjust offset so that the cursor is just inside the visible window.
        if y < self.offset.y {
            self.offset.y = y;
        } else if y >= self.offset.y.saturating_add(height) {
            self.offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < self.offset.x {
            self.offset.x = x;
        } else if x >= self.offset.x.saturating_add(width) {
            self.offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.size().height.saturating_sub(1) as usize;
        let width = self.terminal.size().width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                // Prevent the cursor from keep going down after the last row.
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y };
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
