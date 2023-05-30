use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Device, Nvml};

use super::{MonitorData, MonitorKind};


pub struct GpuData {
    name: String,
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

        let mut gpus = Vec::new();

        if let Some(count) = gpus_count.ok() {
            for i in 0..count {
                let device = nvidia.device_by_index(i);

                if let Some(device) = device.ok() {
                    let gpu = Self::format_nvidia_gpu_data(device);

                    gpus.push(gpu);
                }
            }
        }

        gpus
    }

    fn format_nvidia_gpu_data(device: Device) -> GpuData {
        let name = device.name().unwrap_or_default();

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
            name,
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
        let data = vec![];

        MonitorData {
            kind: MonitorKind::Gpu,
            data,
        }
    }
}