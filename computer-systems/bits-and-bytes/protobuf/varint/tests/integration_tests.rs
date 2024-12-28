use varint;

#[test]
fn test_0() {
    let mut bytes: Vec<u8> = vec![0; varint::MAX_NUM_ENCODABLE_BYTES_FOR_U64];
    let initial: u64 = 0;
    varint::encode(initial, &mut bytes);
    assert_eq!(bytes, vec![0x00; varint::MAX_NUM_ENCODABLE_BYTES_FOR_U64]);
    let decoded = varint::decode(&bytes);
    assert_eq!(decoded, initial);
}

#[test]
fn test_1() {
    let mut bytes: Vec<u8> = vec![0; varint::MAX_NUM_ENCODABLE_BYTES_FOR_U64];
    let initial: u64 = 1;
    varint::encode(initial, &mut bytes);
    assert_eq!(
        bytes,
        vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
    let decoded = varint::decode(&bytes);
    assert_eq!(decoded, initial);
}

#[test]
fn test_150() {
    let mut bytes: Vec<u8> = vec![0; varint::MAX_NUM_ENCODABLE_BYTES_FOR_U64];
    let initial: u64 = 150;
    varint::encode(initial, &mut bytes);
    assert_eq!(
        bytes,
        vec![0x96, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    );
    let decoded = varint::decode(&bytes);
    assert_eq!(decoded, initial);
}

#[test]
fn test_max_int() {
    let mut bytes: Vec<u8> = vec![0; varint::MAX_NUM_ENCODABLE_BYTES_FOR_U64];
    let initial: u64 = u64::MAX;
    varint::encode(initial, &mut bytes);
    assert_eq!(
        bytes,
        vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01]
    );
    let decoded = varint::decode(&bytes);
    assert_eq!(decoded, initial);
}
