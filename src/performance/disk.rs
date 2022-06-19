// TODO: read & write speed

use std::path::Path;
use psutil::disk;
use psutil::disk::{DiskUsage, Partition};
use crate::console::byteutils::convert_bytes_to_giga_u32;

pub struct DiskPartitionUsage {
    pub mount: String,
    pub volume: String,
    pub total: u32,
    pub usage: u32,
    pub percent: f32
}

pub fn get_disk_partitions_usage() -> Vec<DiskPartitionUsage> {

    let partitions = get_disk_partitions();

    partitions.iter()
        .map(|part| {
            let disk_usage = get_disk_usage(part.mountpoint());
            DiskPartitionUsage {
                mount: String::from(part.device()),
                volume: String::from(part.mountpoint().to_str().unwrap_or("")),
                total: convert_bytes_to_giga_u32(disk_usage.total()),
                usage: convert_bytes_to_giga_u32(disk_usage.used()),
                percent: disk_usage.percent()
            }
        })
        .collect()
}

pub fn get_disk_partitions() -> Vec<Partition> {
    disk::partitions_physical().unwrap()
}

pub fn get_disk_usage(mountpoint: &Path) -> DiskUsage {
    disk::disk_usage(mountpoint).unwrap()
}
