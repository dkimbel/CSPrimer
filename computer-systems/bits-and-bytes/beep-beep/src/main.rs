use crossterm::terminal::enable_raw_mode;
use std::io::{self, Read, Write};

const BELL_CHAR: u8 = 0x7;

fn main() {
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut buffer: [u8; 1] = [0; 1];

    loop {
        io::stdin()
            .read(&mut buffer)
            .expect("Failed to read from stdin");

        if buffer[0] == 0x3 {
            // handle ctrl-c (at least on macOS)
            return;
        }
        if let Some(digit) = (buffer[0] as char).to_digit(10) {
            // macOS is only willing to play five bells at a time; we'll sometimes
            // try to send as many as nine, but only five will play
            let bells = vec![BELL_CHAR; digit as usize];
            io::stdout()
                .write_all(&bells)
                .and_then(|_| io::stdout().flush())
                .expect("Failed to write or flush stdout");
        }
    }
}
