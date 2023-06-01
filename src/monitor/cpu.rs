use sysinfo::{CpuExt, System, SystemExt};
use crate::monitor::Component;

use super::{MonitorData, MonitorKind};

pub struct CpuData {
    pub brand: String,
    pub usage_total: f32,
    pub frequency_total: u64,
    pub usage: Vec<f32>,
    pub frequency: Vec<u64>,
}

impl CpuData {
    pub fn new(sys: &System) -> CpuData {
        let cpus = sys.cpus();
        let brand = sys.global_cpu_info().brand();
        let frequency_total = sys.global_cpu_info().frequency();
        let usage_total = sys.global_cpu_info().cpu_usage();

        let (usage, frequency): (Vec<f32>, Vec<u64>) = cpus
            .into_iter()
            .map(|cpu| (cpu.cpu_usage(), cpu.frequency()))
            .unzip();

        CpuData {
            brand: String::from(brand),
            frequency_total,
            usage_total,
            usage,
            frequency,
        }
    }

    pub fn format(&self) -> MonitorData {
        let data = vec![
            Component {
                id: String::from("cpu"),
                name: String::from("CPU Usage"),
                data: format!("{:.2} %", self.usage_total)
            },
            Component {
                id: String::from("cpu-freq"),
                name: String::from("CPU Frequency"),
                data: format!("{:.2} MHz", self.frequency_total)
            }
        ];

        MonitorData {
            kind: MonitorKind::Cpu,
            data,
        }
    }
}