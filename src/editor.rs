use crate::Document;
use crate::Row;
use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    document: Document,
    /// Where of the file the user is currently scrolled to.
    offset: Position,
    cursor_position: Position,
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let document = if args.len() > 1 {
            let filename = &args[1];
            Document::open(filename).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal"),
            document,
            offset: Position::default(),
            // top-left corner
            cursor_position: Position::default(),
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
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            Editor::farewell();
        } else {
            self.draw_rows();
            let cursor_pos_relative_to_offset = Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            };
            Terminal::cursor_position(&cursor_pos_relative_to_offset);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn farewell() {
        println!("Goodbye.\r");
    }

    /// If the row exists, draw it.
    /// Otherwise, draw a tilde, meaning that row is not part of the document and
    /// can't contain any text.
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        // The last line is kept empty for the status bar.
        for term_row in 0..height - 1 {
            Terminal::clear_current_line();
            // If such row exists, draw it.
            if let Some(row) = self.document.row(term_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && term_row == height / 3 {
                // XXX: Should we draw the welcome message if we do open an empty file?
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

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = start + width;
        let row = row.render(start, end);
        println!("{row}\r");
    }

    /// Where the handling logics go.
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Getting a `quit` signal isn't an error.
            Key::Ctrl('q') => self.should_quit = true,
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
        let term_height = self.terminal.size().height as usize;
        // The cursor is allowed to move to the last row of the document.
        let doc_height = self.document.len();
        let mut row_width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                // Prevent the cursor from keep going down after the last row.
                if y < doc_height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    // Left at the beginning of the line moves to the end of the previous line.
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Right => {
                // Right at the end of the line moves to the beginning of the next line.
                if x < row_width {
                    x += 1;
                } else if y < doc_height {
                    y += 1;
                    x = 0;
                }
            }
            Key::PageUp => {
                y = {
                    if y > term_height {
                        y - term_height
                    } else {
                        0
                    }
                }
            }
            Key::PageDown => {
                y = {
                    if y.saturating_add(term_height) < doc_height {
                        y + term_height
                    } else {
                        doc_height
                    }
                }
            }
            Key::Home => x = 0,
            Key::End => x = row_width,
            _ => (),
        }
        // Users may move the cursor from a long line to a short line.
        // We have to prevent the cursor from going beyond the end of the line.
        row_width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > row_width {
            x = row_width;
        }

        self.cursor_position = Position { x, y };
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
