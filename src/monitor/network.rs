use sysinfo::{NetworkExt, System, SystemExt};
use crate::monitor::Component;
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
        let data = networks.into_iter().map(|network| {
            vec![
                Component {
                    id: format!("download-{}", network.interface),
                    name: format!("Download {} Interface", network.interface),
                    data: get_bytevalue_from(network.download_total)
                },
                Component {
                    id: format!("upload-{}", network.interface),
                    name: format!("Upload {} Interface", network.interface),
                    data: get_bytevalue_from(network.upload_total)
                }
            ]
        }).collect::<Vec<Vec<Component>>>().concat();

        MonitorData {
            kind: MonitorKind::Network,
            data
        }
    }
}