use std::io::{self, Read, Write};

const BELL_CHAR: u8 = 0x7;

fn main() {
    let mut buffer: [u8; 1] = [0; 1];

    loop {
        io::stdin()
            .read(&mut buffer)
            .expect("Failed to read from stdin");

        if let Some(digit) = (buffer[0] as char).to_digit(10) {
            let bells = vec![BELL_CHAR; digit as usize];
            io::stdout()
                .write_all(&bells)
                .and_then(|_| io::stdout().flush())
                .expect("Failed to write or flush stdout");
        }
    }
}
