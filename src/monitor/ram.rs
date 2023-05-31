use sysinfo::{System, SystemExt};
use crate::monitor::Component;
use crate::views::byteutils::{from_f64_to_giga};

use super::{MonitorData, MonitorKind};

pub struct RamData {
    pub ram_available: f64,
    pub ram_total: f64,
    pub ram_used: f64,
    pub swap_available: f64,
    pub swap_total: f64,
    pub swap_used: f64,
}

impl RamData {
    pub fn new(sys: &System) -> RamData {
        let ram_total = sys.total_memory() as f64;
        let ram_available = sys.free_memory() as f64;
        let ram_used = sys.used_memory() as f64;

        let swap_total = sys.total_swap() as f64;
        let swap_available = sys.free_swap() as f64;
        let swap_used = sys.used_swap() as f64;

        RamData {
            ram_available,
            ram_total,
            ram_used,
            swap_available,
            swap_total,
            swap_used
        }
    }

    pub fn format(&self) -> MonitorData {
        let data = vec![
            Component {
                id: String::from("ram-usage"),
                name: String::from("RAM Usage"),
                data: format!("{:.2} GB", from_f64_to_giga(self.ram_used))
            },
            Component {
                id: String::from("ram-total"),
                name: String::from("RAM Total"),
                data: format!("{:.2} GB", from_f64_to_giga(self.ram_total))
            },
            Component {
                id: String::from("ram-available"),
                name: String::from("RAM Available"),
                data: format!("{:.2} GB", from_f64_to_giga(self.ram_available))
            },
            Component {
                id: String::from("swap-usage"),
                name: String::from("Swap Usage"),
                data: format!("{:.2} GB", from_f64_to_giga(self.swap_used))
            },
            Component {
                id: String::from("swap-total"),
                name: String::from("Swap Total"),
                data: format!("{:.2} GB", from_f64_to_giga(self.swap_total))
            },
            Component {
                id: String::from("swap-available"),
                name: String::from("Swap Available"),
                data: format!("{:.2} GB", from_f64_to_giga(self.swap_available))
            },
        ];

        MonitorData {
            kind: MonitorKind::Ram,
            data,
        }
    }
}