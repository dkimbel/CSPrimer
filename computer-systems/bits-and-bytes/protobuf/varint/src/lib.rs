const SEVEN_BITS_U64: u64 = 128;
const SEVEN_BITS: u8 = SEVEN_BITS_U64 as u8;

pub const MAX_NUM_ENCODABLE_BYTES: usize = 10;

/// Adds encoded bytes to the given slice. Returns the number of bytes added.
pub fn encode(mut n: u64, bytes: &mut [u8]) -> usize {
    let mut num_added_bytes = 0;

    loop {
        let least_significant_bits = n % SEVEN_BITS_U64;
        let most_significant_bits = n / SEVEN_BITS_U64;
        if most_significant_bits == 0 {
            // nothing left to encode, add our last byte WITHOUT setting the 'continutation' bit
            bytes[num_added_bytes] = least_significant_bits as u8;
            break num_added_bytes + 1;
        }

        // we aren't done adding bytes yet, so set the 'continuation' bit
        bytes[num_added_bytes] = least_significant_bits as u8 + SEVEN_BITS;
        num_added_bytes += 1;

        n = most_significant_bits;
    }
}

pub fn decode(bytes: &[u8]) -> u64 {
    let mut n: u64 = 0;

    for (i, byte) in bytes.into_iter().enumerate() {
        let least_significant_bits = (byte % SEVEN_BITS) as u64;
        n += least_significant_bits * (SEVEN_BITS_U64.pow(i as u32));
        if byte < &SEVEN_BITS {
            // no continutation bit in this byte, so don't continue
            return n;
        }
    }

    panic!("Ran out of bytes without finding last byte of varint!")
}
