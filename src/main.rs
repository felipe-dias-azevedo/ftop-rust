use std::thread;
use std::time::Duration;
use psutil;
use psutil::cpu::CpuPercentCollector;

use systemstat::{System, Platform};

const TIME: Duration = Duration::from_secs(1);


fn get_cpu(cpupc: &mut CpuPercentCollector) {
    //let count = psutil::cpu::cpu_count_physical();

    //println!("CPU cores: {}", count);
    println!("CPU total usage: {}%", cpupc.cpu_percent().unwrap());
    println!("CPU usage (cores): {:?}%", cpupc.cpu_percent_percpu().unwrap());
}

fn main() {

    // psutil
    let mut cpupc: CpuPercentCollector = CpuPercentCollector::new().unwrap();

    // systemstat
    let sys = System::new();

    thread::sleep(TIME);

    get_cpu(&mut cpupc);

    thread::sleep(TIME);

    get_cpu(&mut cpupc);

    let usage_perc = psutil::disk::disk_usage("/").unwrap();

    println!("Disk usage: {}%", usage_perc.percent().round());

    thread::sleep(TIME);
    
    match sys.cpu_load() {
        Ok(cpu) => {
            thread::sleep(TIME);
            let usage = cpu.done().unwrap();
            println!("{:?}", usage);
        },
        Err(give) => println!("\nI give up: {}", give)
    }

    match sys.cpu_load_aggregate()
    {
        Ok(cpuload) =>
        {
            thread::sleep(TIME);
            let cpu_use = cpuload.done().unwrap();
            println!("{:?}", cpu_use.user);
        },
        Err(err) => println!("\nERROR CPU LOAD: {}", err)
    }
}
