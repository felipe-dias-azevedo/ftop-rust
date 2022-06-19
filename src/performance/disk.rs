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

    if get_disk_partitions().is_none() {
        return Vec::new();
    }

    let partitions = get_disk_partitions().unwrap();

    partitions.iter()
        .filter(|part| get_disk_usage(part.mountpoint()).is_some())
        .map(|part| {

            let disk_usage = get_disk_usage(part.mountpoint()).unwrap();

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

pub fn get_disk_partitions() -> Option<Vec<Partition>> {
    let partitions = disk::partitions_physical();
    match partitions {
        Ok(v) => Some(v),
        Err(_err) => None
    }
}

pub fn get_disk_usage(mountpoint: &Path) -> Option<DiskUsage> {
    let disk_usage = disk::disk_usage(mountpoint);
    match disk_usage {
        Ok(v) => Some(v),
        Err(_err) => None
    }
}
