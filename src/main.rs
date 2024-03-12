use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
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
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(e) => die(e),
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
