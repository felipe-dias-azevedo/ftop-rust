use sysinfo::{NetworkExt, System, SystemExt};
use crate::views::byteutils::get_bytevalue_from;

use super::{MonitorData, MonitorKind};

pub struct NetworkData {
    pub interface: String,
    pub download_total: u64,
    pub upload_total: u64
}

impl NetworkData {
    pub fn new(sys: &System) -> Vec<NetworkData> {
        let networks = sys.networks();

        networks.into_iter().map(|(interface, data)| {
            NetworkData {
                interface: String::from(interface),
                download_total: data.received(),
                upload_total: data.transmitted()
            }
        }).collect()
    }

    pub fn format(networks: Vec<NetworkData>) -> MonitorData {
        let data = vec![];

        MonitorData {
            kind: MonitorKind::Network,
            data,
        }
    }
}