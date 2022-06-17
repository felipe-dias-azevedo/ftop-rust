// TODO: temperature, frequency (per cpu and total)

use psutil::cpu::CpuPercentCollector;

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
