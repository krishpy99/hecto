use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        // As long as this variable is alive, we are in raw mode.
        // For information on what are terminal modes, see
        // https://www.gnu.org/software/mit-scheme/documentation/stable/mit-scheme-ref/Terminal-Mode.html.
        let _raw_stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(e) = self.process_keypress() {
                die(e);
            }
            if self.should_quit {
                break;
            }
        }
    }

    pub fn default() -> Self {
        Self { should_quit: false }
    }

    /// Where the handling logics go.
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
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

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
