use crossterm::terminal::enable_raw_mode;
use std::io::{self, Read, Write};
use termios::{tcsetattr, Termios, TCSADRAIN};

const BELL_CHAR: u8 = 0x7;

fn main() {
    let terminal_attrs = Termios::from_fd(0).expect("Failed to initialize terminal attrs");

    enable_raw_mode().expect("Failed to enable raw mode");

    let mut buffer: [u8; 1] = [0; 1];

    loop {
        if let Err(e) = io::stdin().read(&mut buffer) {
            tcsetattr(0, 0, &terminal_attrs).expect("Failed to restore terminal attrs");
            eprintln!("Failed to read from stdin: {e}");
            std::process::exit(1);
        }

        if buffer[0] == 0x3 {
            // handle ctrl-c (at least on macOS)
            tcsetattr(0, TCSADRAIN, &terminal_attrs).expect("Failed to restore terminal attrs");
            std::process::exit(0);
        }

        if let Some(digit) = (buffer[0] as char).to_digit(10) {
            // macOS is only willing to play five bells at a time; we'll sometimes
            // try to send as many as nine, but only five will play
            let bells = vec![BELL_CHAR; digit as usize];
            if let Err(e) = io::stdout()
                .write_all(&bells)
                .and_then(|_| io::stdout().flush())
            {
                tcsetattr(0, 0, &terminal_attrs).expect("Failed to restore terminal attrs");
                eprintln!("Failed to write or flush to stdout: {e}");
                std::process::exit(1);
            }
        }
    }
}
