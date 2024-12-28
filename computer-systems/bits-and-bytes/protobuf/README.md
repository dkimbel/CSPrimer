# Protobuf Encoding/Decoding

This supports encoding or decoding [Base 128 Varints](https://protobuf.dev/programming-guides/encoding/#varints). While I've
implemented the main logic without any dependencies, I used the `hex` crate in my command line binaries to convert between
raw bytes and UTF-8-encoded strings of hexadecimal characters. (Rust works most naturally with UTF-8 strings when dealing
with stdin and stdout.)

## Usage

This assumes that you have used `git clone` to pull this code down locally, and that you're in the same directory as this
README file.

Decode hex `9601` to the unsigned integer `150`:
```bash
echo 9601 | cargo run --bin decode_varint
```

Encode the unsigned integer `150` to hex `9601`:
```bash
echo 150 | cargo run --bin encode_varint
```

Roundtrip encode and decode the unsigned integer `256` back to itself:
```bash
echo 256 | cargo run --bin encode_varint | cargo run --bin decode_varint
```

Roundtrip decode and encode hex `ff0f` back to itself:
```bash
echo ff0f | cargo run --bin decode_varint | cargo run --bin encode_varint
```

## Running tests locally

```bash
cargo test
```
