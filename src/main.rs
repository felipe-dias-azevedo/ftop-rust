mod performance;

use std::thread;
use std::time::Duration;
use psutil;
use psutil::cpu::CpuPercentCollector;
use psutil::{disk, memory};
use psutil::disk::DiskUsage;
use psutil::network::NetIoCountersCollector;

const TIME: Duration = Duration::from_secs(1);


fn main() {

    // TODO: Accept args from user input to
    // https://crates.io/crates/indicatif
    // https://crates.io/crates/clap
    // https://crates.io/crates/owo-colors
    // LOOK INTO: https://crates.io/crates/crossterm

    let mut cpupc: CpuPercentCollector = CpuPercentCollector::new().unwrap();
    let mut netio = NetIoCountersCollector::default();

    loop {
        thread::sleep(TIME);

        //performance::cpu::get_cpu(&mut cpupc);

        let cpu_percents_percpu = cpupc.cpu_percent_percpu().unwrap();
        let partitions = disk::partitions_physical().unwrap();
        let disk_usages: Vec<DiskUsage> = partitions
            .iter()
            .map(|part| disk::disk_usage(part.mountpoint()).unwrap())
            .collect();
        let virtual_memory = memory::virtual_memory().unwrap();
	    let swap_memory = memory::swap_memory().unwrap();
        let net_io_counters = netio.net_io_counters().unwrap();

        println!("cpu: {:?}", cpu_percents_percpu);
        println!("disk: {:?}", disk_usages);
        println!("ram: {:?}", virtual_memory);
        println!("swap: {:?}", swap_memory);
        println!("net: {:?}", net_io_counters);
        println!()
    }

}
