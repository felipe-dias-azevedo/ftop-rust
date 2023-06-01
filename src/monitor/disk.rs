use sysinfo::{DiskExt, ProcessExt, System, SystemExt, DiskKind};
use crate::monitor::Component;
use crate::views::byteutils::{from_f64_to_giga, get_bytevalue_from};

use super::{MonitorData, MonitorKind};

pub struct DisksData {
    pub disks: Vec<DiskData>,
    pub read_total: Option<u64>,
    pub write_total: Option<u64>,
}

pub struct DiskData {
    pub name: String,
    pub disk_type: Option<String>,
    pub file_system: Option<String>,
    pub mount_point: String,
    pub removable: bool,
    pub space_available: f64,
    pub space_total: f64,
    pub space_used: f64,
}

impl DisksData {
    pub fn new(sys: &System) -> DisksData {
        let disks = sys.disks();

        let disks_data = disks.into_iter().map(|d| {
            let name = d.name();
            let disk_type = d.kind();
            let file_system: &[u8] = d.file_system();
            let space = d.total_space() as f64;
            let available_space = d.available_space() as f64;
            let mount_point = d.mount_point();
            let is_removable = d.is_removable();

            DiskData {
                name: name.to_os_string().into_string().unwrap_or_default(),
                disk_type: match disk_type {
                    DiskKind::SSD => Some(String::from("SSD")),
                    DiskKind::HDD => Some(String::from("HD")),
                    _ => None
                },
                file_system: String::from_utf8(Vec::from(file_system)).ok(),
                space_total: space,
                space_available: available_space,
                space_used: space - available_space,
                mount_point: String::from(mount_point.to_str().unwrap_or_default()),
                removable: is_removable,
            }
        }).collect::<Vec<DiskData>>();

        let disk_usage = sys
            .processes()
            .into_iter()
            .filter_map(|(_pid, process)| {
                let disk_usage = process.disk_usage();
                let read = disk_usage.read_bytes;
                let write = disk_usage.written_bytes;

                if read == 0 && write == 0 {
                    return None;
                }

                return Some((read, write));
            })
            .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1));

        let (read, write) = match disk_usage {
            Some(x) => (Some(x.0), Some(x.1)),
            _ => (None, None)
        };

        DisksData {
            read_total: read,
            write_total: write,
            disks: disks_data,
        }
    }

    pub fn format(&self) -> MonitorData {
        let disks_data = self.disks.iter().map(|disk| {
            vec![
                Component {
                    id: format!("disk-{}-usage", disk.name),
                    name: format!("Disk {} Usage", disk.name),
                    data: format!("{:.2} %", (disk.space_used / disk.space_total) * 100f64),
                },
                Component {
                    id: format!("disk-{}-used", disk.name),
                    name: format!("Disk {} Used", disk.name),
                    data: format!("{:.2} GB", from_f64_to_giga(disk.space_used)),
                },
                Component {
                    id: format!("disk-{}-total", disk.name),
                    name: format!("Disk {} Total", disk.name),
                    data: format!("{:.2} GB", from_f64_to_giga(disk.space_total)),
                },
                Component {
                    id: format!("disk-{}-available", disk.name),
                    name: format!("Disk {} Available", disk.name),
                    data: format!("{:.2} GB", from_f64_to_giga(disk.space_available)),
                }
            ]
        }).collect::<Vec<Vec<Component>>>().concat();

        let disks_general_data = vec![
            Component {
                id: String::from("disk-read"),
                name: String::from("Disk Read Total"),
                data: match self.read_total {
                    Some(x) => get_bytevalue_from(x),
                    _ => get_bytevalue_from(0)
                },
            },
            Component {
                id: String::from("disk-write"),
                name: String::from("Disk Write Total"),
                data: match self.write_total {
                    Some(x) => get_bytevalue_from(x),
                    _ => get_bytevalue_from(0)
                },
            }
        ];

        let data = vec![disks_data, disks_general_data].concat();

        MonitorData {
            kind: MonitorKind::Disk,
            data,
        }
    }
}