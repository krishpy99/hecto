use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    // As long as this variable is alive, we are in raw mode.
    // For information on what are terminal modes, see
    // https://www.gnu.org/software/mit-scheme/documentation/stable/mit-scheme-ref/Terminal-Mode.html.
    let _raw_stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?}\r", b);
                } else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_ctrl(b'q') {
                    break;
                }
            }
            Err(e) => die(e),
        }
    }
}

fn to_ctrl(b: u8) -> u8 {
    b & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}
