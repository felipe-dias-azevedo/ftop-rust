mod performance;

use std::thread;
use std::time::Duration;
use psutil;
use psutil::cpu::CpuPercentCollector;

const TIME: Duration = Duration::from_secs(1);


fn main() {

    // TODO: Accept args from user input to
    // https://crates.io/crates/indicatif
    // https://crates.io/crates/clap
    // https://crates.io/crates/owo-colors
    // LOOK INTO: https://crates.io/crates/crossterm

    let mut cpupc: CpuPercentCollector = CpuPercentCollector::new().unwrap();

    loop {
        thread::sleep(TIME);

        performance::cpu::get_cpu(&mut cpupc);
    }

}
