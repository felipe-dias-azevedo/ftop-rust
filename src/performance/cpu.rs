// TODO: temperature, usage, frequency (per cpu and total)

use psutil::cpu::CpuPercentCollector;

// FIXME: ITS AN EXAMPLE, REMOVE IT.
pub fn get_cpu(cpupc: &mut CpuPercentCollector) {
    //let count = psutil::cpu::cpu_count_physical();

    //println!("CPU cores: {}", count);
    println!("CPU total usage: {}%", cpupc.cpu_percent().unwrap());
    println!("CPU usage (cores): {:?}%", cpupc.cpu_percent_percpu().unwrap());
}