// TODO: temperature, frequency (per cpu and total)

use psutil::cpu::CpuPercentCollector;
use psutil::Error;
use psutil::sensors::TemperatureSensor;
use crate::console;

fn get_core_count() -> usize {
    psutil::cpu::cpu_count_physical() as usize
}

pub fn get_cpu_usage_per_thread(cpupc: &mut CpuPercentCollector) -> Vec<f32> {

    cpupc.cpu_percent_percpu().unwrap_or(vec![0.0; get_core_count()])
}

pub fn get_cpu_usage(cpupc: &mut CpuPercentCollector) -> f32 {

    cpupc.cpu_percent().unwrap_or(0.0)
}

fn cpu_temperature() -> std::thread::Result<Vec<Result<TemperatureSensor, Error>>> {
    console::panicutils::catch_unwind_silent(|| {
        psutil::sensors::temperatures()
    })
}

pub fn get_cpu_temperature() -> Option<u8> {
    let temperatures = match cpu_temperature() {
        Ok(v) => Some(v),
        Err(_err) => None
    };

    if temperatures.is_none() {
        return None
    }

    let temperatures = temperatures.unwrap();

    let temps: Vec<TemperatureSensor> = temperatures.into_iter()
        .filter_map(|p| match p {
            Ok(v) => Some(v),
            _ => None
        })
        .filter(|temp| {
            temp.unit().contains("coretemp") || temp.unit().contains("k10temp")
        })
        .collect();

    match temps.first() {
        Some(v) => Some(v.current().celsius().round() as u8),
        _ => None
    }
}
