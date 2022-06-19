use psutil::memory;
use crate::console::byteutils::convert_bytes_to_giga_f32;

pub struct MemoryUsage {
    pub percent: f32,
    pub total: f32,
    pub usage: f32
}

pub fn get_ram_usage() -> MemoryUsage {

    let ram_usage = memory::virtual_memory().unwrap();

    MemoryUsage {
        usage: convert_bytes_to_giga_f32(ram_usage.used()),
        total: convert_bytes_to_giga_f32(ram_usage.total()),
        percent: ram_usage.percent().round()
    }
}

pub fn get_swap_usage() -> MemoryUsage {

    let swap_usage = memory::swap_memory().unwrap();

    MemoryUsage {
        usage: convert_bytes_to_giga_f32(swap_usage.used()),
        total: convert_bytes_to_giga_f32(swap_usage.total()),
        percent: swap_usage.percent().round()
    }
}