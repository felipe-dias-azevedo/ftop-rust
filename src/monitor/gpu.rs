use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Device, Nvml};
use crate::monitor::Component;
use crate::views::byteutils::from_f64_to_giga;

use super::{MonitorData, MonitorKind};


pub struct GpuData {
    // name: String,
    fans_speed: Vec<u32>,
    memory_used: Option<f64>,
    memory_available: Option<f64>,
    memory_total: Option<f64>,
    power_usage: Option<u32>,
    temperature: Option<u32>,
    usage_gpu: Option<u32>,
    usage_memory: Option<u32>,
}

impl GpuData {
    pub fn new(nvidia: &Result<Nvml, NvmlError>) -> Vec<GpuData> {
        if let Some(nvidia) = nvidia.as_ref().ok() {
            return Self::get_nvidia_gpus(nvidia);
        }

        Vec::new()
    }

    fn get_nvidia_gpus(nvidia: &Nvml) -> Vec<GpuData> {
        let gpus_count = nvidia.device_count();

        if let Some(count) = gpus_count.ok() {
            return (0..count)
                .filter_map(|i| nvidia.device_by_index(i).ok())
                .map(|device| Self::format_nvidia_gpu_data(device))
                .collect();
        }

        Vec::new()
    }

    fn format_nvidia_gpu_data(device: Device) -> GpuData {
        // let name = device.name().unwrap_or_default();

        let fans_speed = match device.num_fans().ok() {
            Some(fans_count) => (0..fans_count)
                .filter_map(|i| device.fan_speed(i).ok())
                .collect::<Vec<u32>>(),
            _ => Vec::new()
        };

        let (memory_used, memory_total, memory_available) = match device.memory_info().ok() {
            Some(mem) => (Some(mem.used as f64), Some(mem.total as f64), Some(mem.free as f64)),
            _ => (None, None, None),
        };
        let power_usage = device.power_usage().ok();
        let temperature = device.temperature(TemperatureSensor::Gpu).ok();
        let (usage_gpu, usage_memory) = match device.utilization_rates().ok() {
            Some(util) => (Some(util.gpu), Some(util.memory)),
            _ => (None, None),
        };

        GpuData {
            // name,
            fans_speed,
            memory_used,
            memory_total,
            memory_available,
            power_usage,
            temperature,
            usage_gpu,
            usage_memory,
        }
    }

    pub fn format(gpus: Vec<GpuData>) -> MonitorData {
        let data = gpus.into_iter()
            .enumerate()
            .map(|(i, gpu)| {
                let gpu_info = vec![
                    Component {
                        id: format!("gpu-{}-power", i),
                        name: format!("GPU {} Power Usage", i),
                        data: match gpu.power_usage {
                            Some(x) => format!("{} W", x / 1_000),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-temp", i),
                        name: format!("GPU {} Temperature", i),
                        data: match gpu.temperature {
                            Some(x) => format!("{} ºC", x),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-vram-usage", i),
                        name: format!("GPU {} VRAM Usage", i),
                        data: match (gpu.memory_used, gpu.memory_total) {
                            (Some(used), Some(total)) => format!("{:.2} %", (used / total) * 100f64),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-vram-used", i),
                        name: format!("GPU {} VRAM Used", i),
                        data: match gpu.memory_used {
                            Some(x) => format!("{:.2} GB", from_f64_to_giga(x)),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-vram-total", i),
                        name: format!("GPU {} VRAM Total", i),
                        data: match gpu.memory_total {
                            Some(x) => format!("{:.2} GB", from_f64_to_giga(x)),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-vram-available", i),
                        name: format!("GPU {} VRAM Available", i),
                        data: match gpu.memory_available {
                            Some(x) => format!("{:.2} GB", from_f64_to_giga(x)),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-usage", i),
                        name: format!("GPU {} Usage", i),
                        data: match gpu.usage_gpu {
                            Some(x) => format!("{} %", x),
                            _ => String::from("")
                        },
                    },
                    Component {
                        id: format!("gpu-{}-mem-usage", i),
                        name: format!("GPU {} Memory Usage", i),
                        data: match gpu.usage_memory {
                            Some(x) => format!("{} %", x),
                            _ => String::from("")
                        },
                    },
                ];

                let fans_info = gpu.fans_speed.into_iter().enumerate().map(|(j, fan)| {
                    Component {
                        id: format!("gpu-{}-fan-{}", i, j),
                        name: format!("GPU {} Fan {} Speed", i, j),
                        data: format!("{} %", fan),
                    }
                }).collect();

                vec![gpu_info, fans_info].concat()
            }).collect::<Vec<Vec<Component>>>().concat();

        MonitorData {
            kind: MonitorKind::Gpu,
            data,
        }
    }
}