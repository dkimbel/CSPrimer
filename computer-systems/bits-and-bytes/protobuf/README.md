# Protobuf Encoding/Decoding

This supports encoding or decoding [Base 128 Varints](https://protobuf.dev/programming-guides/encoding/#varints). While I've
implemented the main logic without any dependencies, I used the `hex` crate in my command line binaries to convert between
raw bytes and UTF-8-encoded strings of hexadecimal characters. (Rust works most naturally with UTF-8 strings when dealing
with stdin and stdout.)

## Usage

```
echo 9601 | cargo run --bin decode_varint
```

```
echo 150 | cargo run --bin encode_varint
```

```
echo 256 | cargo run --bin encode_varint | cargo run --bin decode_varint
```

```
echo ff0f | cargo run --bin decode_varint | cargo run --bin encode_varint
```

## Running tests locally

```
cargo test
```
