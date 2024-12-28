const LSB_MASK: u8 = 0b01_11_11_11;
const MSB_MASK: u8 = !LSB_MASK; // 0b10_00_00_00
const LSB_MASK_U64: u64 = LSB_MASK as u64;

pub const MAX_NUM_ENCODABLE_BYTES_FOR_U64: usize = 10;

/// Adds encoded bytes to the given slice. Returns the number of bytes added.
pub fn encode(n: u64, bytes: &mut [u8]) -> usize {
    let mut num_added_bytes = 0;
    let mut remaining_bits = n;

    loop {
        let least_significant_bits = remaining_bits & LSB_MASK_U64;
        remaining_bits >>= 7;

        if remaining_bits == 0 {
            // Nothing left to encode, add our last byte WITHOUT setting the 'continuation' bit
            bytes[num_added_bytes] = least_significant_bits as u8;
            break num_added_bytes + 1;
        }

        // We aren't done adding bytes yet, so set the 'continuation' bit
        bytes[num_added_bytes] = least_significant_bits as u8 | MSB_MASK;
        num_added_bytes += 1;
    }
}

/// Reads through the given bytes slice until it finds a byte without a continuation bit,
/// effectively decoding the first varint that can be found. Panics if the slice is exhausted
/// before encountering a byte without a continuation bit.
/// NOTE: this could be updated to also return the number of bytes that were decoded.
pub fn decode(bytes: &[u8]) -> u64 {
    let mut n: u64 = 0;
    let mut num_bits_already_added = 0;

    // We're iterating left-to-right through our bytes; this lets us easily work our way
    // towards the first 'ending' byte (without a continuation bit), even if the bytes encode
    // many varints. HOWEVER, this is a little awkward for decoding, because each byte is
    // increasingly significant to our decoded int. Our first iteration yields our decoded
    // int's least significant seven bits, then our second iteration yields the next-least-
    // significant seven, and so on. To make this work, each iteration uses a bit mask with
    // more and more zero-padding on its right.
    for byte in bytes.iter() {
        let least_significant_bits = (byte & LSB_MASK) as u64;
        let new_bits_mask = least_significant_bits << num_bits_already_added;
        n |= new_bits_mask;
        num_bits_already_added += 7;

        if byte < &MSB_MASK {
            // no continuation bit in this byte, so don't continue
            return n;
        }
    }

    panic!("Ran out of bytes without finding last byte of varint!")
}
