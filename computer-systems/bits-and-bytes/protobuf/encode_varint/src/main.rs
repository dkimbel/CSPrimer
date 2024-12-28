use hex;
use std::io;
use varint;

const INPUT_ERROR_MSG: &str = "please provide an unsigned 64-bit integer to encode via stdin";

fn main() {
    let mut unparsed_n = String::new();
    io::stdin()
        .read_line(&mut unparsed_n)
        .expect(INPUT_ERROR_MSG);
    let n = unparsed_n.trim().parse::<u64>().expect(INPUT_ERROR_MSG);
    let mut bytes: Vec<u8> = vec![0; varint::MAX_NUM_ENCODABLE_BYTES_FOR_U64];

    let num_bytes = varint::encode_v2(n, &mut bytes);

    // we output bytes to stdout as UTF-8 formatted hexadecimal (terminated by a newline)
    let utf8_encoded_hex = hex::encode(&bytes[0..num_bytes]);
    println!("{utf8_encoded_hex}");
}
