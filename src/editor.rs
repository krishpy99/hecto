use crate::terminal::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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

    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide(); // prevent the cursor from blinking
        Terminal::cursor_position(0, 0);
        if self.should_quit {
            Terminal::clear_screen();
            Editor::farewell();
        } else {
            self.draw_rows();
            Terminal::cursor_position(0, 0);
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
            _ => Ok(()),
        }
    }
}

fn die(e: &std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
