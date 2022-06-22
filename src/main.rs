mod performance;
mod console;

use std::thread;
use std::time::Duration;
use psutil::cpu::CpuPercentCollector;
use psutil::network::NetIoCountersCollector;

const TIME: Duration = Duration::from_secs(1);
const PER_CPU: bool = false;
const MARGIN: f32 = 0.5;

fn main() {

    // TODO: Accept args from user input to
    // https://crates.io/crates/indicatif
    // https://crates.io/crates/clap
    // https://crates.io/crates/owo-colors
    // LOOK INTO: https://crates.io/crates/crossterm

    let console = console::displayutils::ConsoleDisplay::new();
    let mut cpupc: CpuPercentCollector = CpuPercentCollector::new().unwrap();
    let mut netio: NetIoCountersCollector = NetIoCountersCollector::default();

    loop {
        let first_net_usage = performance::net::get_net_usage(&mut netio);

        thread::sleep(TIME);

        let cpu_temperature = performance::cpu::get_cpu_temperature();
        match cpu_temperature {
            Some(temp) => println!("CPU Temperature: {} ºC", temp),
            _ => {}
        };

        if PER_CPU {
            let cpu_usage = performance::cpu::get_cpu_usage_per_thread(&mut cpupc);
            for core in 0..cpu_usage.len() {
                println!("CPU {} Usage: {:.0}%",
                         core + 1, cpu_usage[core]);
                println!("{}", console.get_loadbar(cpu_usage[core], MARGIN));
            }
        } else {
            let cpu_usage = performance::cpu::get_cpu_usage(&mut cpupc);
            println!("CPU Usage: {:.0}%",
                     cpu_usage);
            println!("{}", console.get_loadbar(cpu_usage, MARGIN));
        }

        let disk_usage = performance::disk::get_disk_partitions_usage();
        for partition in disk_usage {
            println!("Disk {} Usage at {}: {:.1} GB of {:.1} GB Total",
                     partition.volume, partition.mount, partition.usage, partition.total);
            println!("{}", console.get_loadbar(partition.percent, MARGIN));
        }

        let ram_usage = performance::mem::get_ram_usage();
        println!("Ram Usage: {:.2} GB of {:.2} GB",
                 ram_usage.usage, ram_usage.total);
        println!("{}", console.get_loadbar(ram_usage.percent, MARGIN));

        let swap_usage = performance::mem::get_swap_usage();
        println!("Swap Usage: {:.2} GB of {:.2} GB",
                 swap_usage.usage, swap_usage.total);
        println!("{}", console.get_loadbar(swap_usage.percent, MARGIN));

        let second_net_usage = performance::net::get_net_usage(&mut netio);
        let net_usage = performance::net::calc_net_interval(first_net_usage, second_net_usage);
        println!("Download Rate: {} {}",
                 net_usage.download.value, net_usage.download.types);
        println!("Upload Rate: {} {}",
                 net_usage.upload.value, net_usage.upload.types);

        println!()
    }

}
