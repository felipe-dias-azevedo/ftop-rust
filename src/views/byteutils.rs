const KILO: u16 = 2u16.pow(10);
const MEGA: u32 = 2u32.pow(20);
const GIGA: u32 = 2u32.pow(30);

pub fn to_kilo(bytes: u64) -> u32 {
    (bytes / KILO as u64) as u32
}

pub fn to_mega(bytes: u64) -> u32 {
    (bytes / MEGA as u64) as u32
}

pub fn to_giga(bytes: u64) -> u32 {
    (bytes / GIGA as u64) as u32
}

pub fn from_f64_to_giga(bytes: f64) -> f64 {
    (bytes / GIGA as f64) as f64
}

pub fn get_bytevalue_from(bytes: u64) -> String {
    match bytes {
        1_000 ..= 999_999 => format!("{} KB/s", to_kilo(bytes)),
        1_000_000 ..= 999_999_999 => format!("{} MB/s", to_mega(bytes)),
        byte if byte >= 1_000_000_000 => format!("{} GB/s", to_giga(bytes)),
        _ => format!("{} B/s", bytes as u32)
    }
}