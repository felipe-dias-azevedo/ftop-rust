pub struct ByteValue {
    pub value: u64,
    pub types: String
}

pub fn convert_bytes_to_giga_u64(bytes: u64) -> u64 {
    bytes / 1_000_000_000
}

pub fn convert_bytes_to_giga_f32(bytes: u64) -> f32 {
    bytes as f32 / 1_000_000_000.0
}

pub fn convert_value_from(bytes: u64) -> ByteValue {
    match bytes {
        1_000 ..= 999_999 => ByteValue {
            value: bytes / 1_000,
            types: String::from("KB/s")
        },
        1_000_000 ..= 999_999_999 => ByteValue {
            value: bytes / 1_000_000,
            types: String::from("MB/s")
        },
        byte if byte >= 1_000_000_000 => ByteValue {
            value: convert_bytes_to_giga_u64(bytes),
            types: String::from("GB/s")
        },
        _ => ByteValue {
            value: bytes,
            types: String::from("Bytes/s")
        }
    }
}