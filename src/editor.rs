use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        // As long as this variable is alive, we are in raw mode.
        // For information on what are terminal modes, see
        // https://www.gnu.org/software/mit-scheme/documentation/stable/mit-scheme-ref/Terminal-Mode.html.
        let _raw_stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        if c.is_control() {
                            println!("{:?}\r", c as u8);
                        } else {
                            println!("{:?} ({c})\r", c as u8);
                        }
                    }
                    Key::Ctrl('q') => break,
                    _ => println!("{key:?}\r"),
                },
                Err(e) => die(e),
            }
        }
    }

    pub fn default() -> Self {
        Self {}
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
