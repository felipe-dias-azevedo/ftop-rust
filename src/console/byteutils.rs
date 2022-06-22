const KILO: u16 = 2u16.pow(10);
const MEGA: u32 = 2u32.pow(20);
const GIGA: u32 = 2u32.pow(30);

pub struct ByteValue {
    pub value: u32,
    pub types: String
}

fn convert_bytes_to_kilo_u32(bytes: u64) -> u32 {
    (bytes / KILO as u64) as u32
}

fn convert_bytes_to_mega_u32(bytes: u64) -> u32 {
    (bytes / MEGA as u64) as u32
}

pub fn convert_bytes_to_giga_u32(bytes: u64) -> u32 {
    (bytes / GIGA as u64) as u32
}

pub fn convert_bytes_to_giga_f32(bytes: u64) -> f32 {
    (bytes as f64 / GIGA as f64) as f32
}

pub fn get_bytevalue_from(bytes: u64) -> ByteValue {
    match bytes {
        1_000 ..= 999_999 => ByteValue {
            value: convert_bytes_to_kilo_u32(bytes),
            types: String::from("KB/s")
        },
        1_000_000 ..= 999_999_999 => ByteValue {
            value: convert_bytes_to_mega_u32(bytes),
            types: String::from("MB/s")
        },
        byte if byte >= 1_000_000_000 => ByteValue {
            value: convert_bytes_to_giga_u32(bytes),
            types: String::from("GB/s")
        },
        _ => ByteValue {
            value: bytes as u32,
            types: String::from("Bytes/s")
        }
    }
}