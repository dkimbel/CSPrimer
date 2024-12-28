use hex;
use std::io;
use varint;

const INPUT_ERROR_MSG: &str = "please provide a UTF-8-encoded hexadecimal string via stdin";

fn main() {
    let mut unparsed_bytes = String::new();
    io::stdin()
        .read_line(&mut unparsed_bytes)
        .expect(INPUT_ERROR_MSG);
    let bytes = hex::decode(unparsed_bytes.trim()).expect(INPUT_ERROR_MSG);

    let n = varint::decode(&bytes);
    println!("{n}");
}
