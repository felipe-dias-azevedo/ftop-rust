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

pub fn get_disk_partitions_usage() -> Vec<DiskPartitionUsage> {

    let disk_partitions = get_disk_partitions();

    if disk_partitions.is_none() {
        return Vec::new();
    }

    let partitions = disk_partitions.unwrap();

    partitions.into_iter()
        .filter_map(|partition| match get_disk_usage(partition.mountpoint()) {
            Some(disk_usage) => {
                let disk_partition_usage = DiskPartitionUsage {
                    mount: String::from(partition.device()),
                    volume: String::from(partition.mountpoint().to_str().unwrap_or("")),
                    total: convert_bytes_to_giga_f32(disk_usage.total()),
                    usage: convert_bytes_to_giga_f32(disk_usage.used()),
                    percent: disk_usage.percent()
                };

                Some(disk_partition_usage)
            },
            _ => None
        })
        .collect()
}

pub fn get_disk_partitions() -> Option<Vec<Partition>> {
    let partitions = disk::partitions_physical();

    match partitions {
        Ok(v) => match SYS {
            "macos" => {
                let partitions = v.into_iter()
                    .filter(| part | {
                         part.mountpoint().to_str().unwrap_or("") == "/"
                    })
                    .collect();

                Some(partitions)
            }
            "linux" => {
                let partitions = v.into_iter()
                    .filter(|part| {
                        !part.mountpoint().to_str().unwrap_or("")
                            .contains("boot")
                    })
                    .collect();

                Some(partitions)
            }
            _ => Some(v)
        },
        _ => None
    }
}

pub fn get_disk_usage(mountpoint: &Path) -> Option<DiskUsage> {
    disk::disk_usage(mountpoint).ok()
}
