// TODO: temperature, frequency (per cpu and total)

use psutil::cpu::CpuPercentCollector;
use psutil::Error;
use psutil::sensors::TemperatureSensor;
use crate::console;

//const CORE_COUNT: usize = psutil::cpu::cpu_count_physical() as usize;

pub fn get_cpu_usage_per_thread(cpupc: &mut CpuPercentCollector) -> Vec<u8> {

    let usage = cpupc.cpu_percent_percpu().unwrap();

    usage.iter()
        .map(|val| val.round() as u8)
        .collect()
}

pub fn get_cpu_usage(cpupc: &mut CpuPercentCollector) -> u8 {

    cpupc.cpu_percent().unwrap() as u8
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

    let temps: Vec<&TemperatureSensor> = temperatures
        .iter()
        .filter(|&val| val.is_ok())
        .map(|val| val.as_ref().unwrap())
        .filter(|temp| temp.unit().contains("coretemp"))
        .collect();

    match temps.first() {
        Some(&v) => Some(v.current().celsius().round() as u8),
        _ => None
    }
}
