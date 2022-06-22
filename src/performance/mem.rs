use psutil::memory;
use crate::console::byteutils::convert_bytes_to_giga_f32;

pub struct MemoryUsage {
    pub percent: f32,
    pub total: f32,
    pub usage: f32
}

pub fn get_ram_usage() -> Option<MemoryUsage> {

    let ram_usage = memory::virtual_memory().ok();

    if ram_usage.is_none() {
        return None;
    }

    let ram_usage = ram_usage.unwrap();

    let ram_usage = MemoryUsage {
        usage: convert_bytes_to_giga_f32(ram_usage.used()),
        total: convert_bytes_to_giga_f32(ram_usage.total()),
        percent: ram_usage.percent()
    };

    Some(ram_usage)
}

pub fn get_swap_usage() -> Option<MemoryUsage> {

    let swap_usage = memory::swap_memory().ok();

    if swap_usage.is_none() {
        return None;
    }

    let swap_usage = swap_usage.unwrap();

    let swap_usage = MemoryUsage {
        usage: convert_bytes_to_giga_f32(swap_usage.used()),
        total: convert_bytes_to_giga_f32(swap_usage.total()),
        percent: swap_usage.percent()
    };

    Some(swap_usage)
}