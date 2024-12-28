mod bitwise;
mod division;

pub use division::decode as decode_v1;
pub use division::encode as encode_v1;

pub use bitwise::decode as decode_v2;
pub use bitwise::encode as encode_v2;
pub use bitwise::MAX_NUM_ENCODABLE_BYTES_FOR_U64;
