mod performance;
mod byteutils;

use std::thread;
use std::time::Duration;
use psutil::cpu::CpuPercentCollector;
use psutil::network::NetIoCountersCollector;

const TIME: Duration = Duration::from_secs(1);
const PER_CPU: bool = true;

fn main() {

    // TODO: Accept args from user input to
    // https://crates.io/crates/indicatif
    // https://crates.io/crates/clap
    // https://crates.io/crates/owo-colors
    // LOOK INTO: https://crates.io/crates/crossterm

    let mut cpupc: CpuPercentCollector = CpuPercentCollector::new().unwrap();
    let mut netio: NetIoCountersCollector = NetIoCountersCollector::default();

    loop {
        thread::sleep(TIME);

        if PER_CPU {
            let cpu_usage = performance::cpu::get_cpu_usage_per_thread(&mut cpupc);
            for core in 0..cpu_usage.len() {
                println!("CPU {} Usage: {}%", core + 1, cpu_usage[core]);
            }
        } else {
            let cpu_usage = performance::cpu::get_cpu_usage(&mut cpupc);
            println!("CPU Usage: {}%", cpu_usage);
        }

        let disk_usage = performance::disk::get_disk_partitions_usage();
        for partition in disk_usage {
            println!("Disk {} Usage at {}: {} GB of {} GB Total",
                     partition.volume, partition.mount, partition.usage, partition.total);
        }

        let ram_usage = performance::mem::get_ram_usage();
        println!("Ram Usage: {:.2} GB of {:.2} GB", ram_usage.usage, ram_usage.total);

        let swap_usage = performance::mem::get_swap_usage();
        println!("Swap Usage: {:.2} GB of {:.2} GB", swap_usage.usage, swap_usage.total);

        let net_usage = performance::net::get_net_usage(&mut netio);
        println!("Download Rate: {} {}", net_usage.download.value, net_usage.download.types);
        println!("Upload Rate: {} {}", net_usage.upload.value, net_usage.upload.types);

        println!()
    }

}
