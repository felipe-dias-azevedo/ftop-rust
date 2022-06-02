use std::thread;
use std::time::Duration;
use psutil;
use psutil::cpu::CpuPercentCollector;

const TIME: Duration = Duration::from_secs(1);


fn get_cpu(cpupc: &mut CpuPercentCollector) {
    //let count = psutil::cpu::cpu_count_physical();

    //println!("CPU cores: {}", count);
    println!("CPU total usage: {}%", cpupc.cpu_percent().unwrap());
    println!("CPU usage (cores): {:?}%", cpupc.cpu_percent_percpu().unwrap());
}

fn main() {

    let mut cpupc: CpuPercentCollector = CpuPercentCollector::new().unwrap();

    thread::sleep(TIME);

    get_cpu(&mut cpupc);

    thread::sleep(TIME);

    get_cpu(&mut cpupc);

    let usage_perc = psutil::disk::disk_usage("/").unwrap();

    println!("Disk usage: {}%", usage_perc.percent().round());

    thread::sleep(TIME);
}
