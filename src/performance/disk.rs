// TODO: read & write speed

use std::path::Path;
use psutil::disk;
use psutil::disk::{DiskUsage, Partition};
use crate::console::byteutils::convert_bytes_to_giga_f32;

const SYS: &str = std::env::consts::OS;

pub struct DiskPartitionUsage {
    pub mount: String,
    pub volume: String,
    pub total: f32,
    pub usage: f32,
    pub percent: f32
}

fn get_disk_usage_by(partition: Partition) -> DiskPartitionUsage {
    let disk_usage = get_disk_usage(partition.mountpoint()).unwrap();

    DiskPartitionUsage {
        mount: String::from(partition.device()),
        volume: String::from(partition.mountpoint().to_str().unwrap_or("")),
        total: convert_bytes_to_giga_f32(disk_usage.total()),
        usage: convert_bytes_to_giga_f32(disk_usage.used()),
        percent: disk_usage.percent()
    }
}

pub fn get_disk_partitions_usage() -> Vec<DiskPartitionUsage> {

    if get_disk_partitions().is_none() {
        return Vec::new();
    }

    let partitions = get_disk_partitions().unwrap();

    partitions.into_iter()
        .filter(|part| get_disk_usage(part.mountpoint()).is_some())
        .map(get_disk_usage_by)
        .collect()
}

pub fn get_disk_partitions() -> Option<Vec<Partition>> {
    let partitions = disk::partitions_physical();

    match partitions {
        Ok(v) => {
            if SYS != "macos" {
                return Some(v)
            }

            let partitions = v.into_iter()
                .filter(| part | "/" == part.mountpoint().to_str().unwrap_or(""))
                .collect();

            Some(partitions)
        },
        _ => None
    }
}

pub fn get_disk_usage(mountpoint: &Path) -> Option<DiskUsage> {
    disk::disk_usage(mountpoint).ok()
}
