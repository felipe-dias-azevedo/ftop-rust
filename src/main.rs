mod performance;
mod console;

use std::borrow::Borrow;
use std::thread;
use std::time::Duration;
use psutil::cpu::CpuPercentCollector;
use psutil::network::NetIoCountersCollector;
use psutil::sensors::TemperatureSensor;

const TIME: Duration = Duration::from_secs(1);
const PER_CPU: bool = false;

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

        let cpu_temperature = performance::cpu::get_cpu_temperature();
        match cpu_temperature {
            Some(temp) => println!("CPU Temperature: {} ºC", temp),
            _ => {}
        };

        // for x in temps {
        //     println!("temperature: {:?}", x);
        // }


        println!()
    }

}
