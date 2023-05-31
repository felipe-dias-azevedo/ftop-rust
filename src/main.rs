mod monitor;
mod views;

use std::{io, thread};
use std::time::Duration;
use nvml_wrapper::Nvml;
use sysinfo::{System, SystemExt};
use clap::Parser;

use crate::{monitor::{get_system_data, get_components_data}, views::displayutils::{start_message, list_components}};
use crate::monitor::get_components_filtered;
use crate::views::displayutils::{components_show, Display};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List all components available
    #[arg(short, long, default_value_t = false)]
    list_components: bool,

    /// Components to monitoring
    #[arg(
        short,
        long,
        value_parser,
        num_args = 1..,
        value_delimiter = ' ',
        default_values_t = vec![String::from("cpu"), String::from("ram-usage")]
    )]
    components: Vec<String>,

    /// Update frequency time (in seconds)
    #[arg(short, long, default_value_t = 1)]
    update: u8
}

fn main() {

    // TODO: Accept args from user input to
    // https://crates.io/crates/indicatif
    // https://crates.io/crates/clap
    // https://crates.io/crates/owo-colors
    // LOOK INTO: https://crates.io/crates/crossterm

    let nvidia = Nvml::init();
    let mut sys = System::new_all();

    let args = Args::parse();

    if args.list_components {

        let components = get_components_data(&sys, &nvidia);

        list_components(components);
        return;
    }
    
    let system_data = get_system_data(&sys);
    
    start_message(&system_data.system);
    thread::sleep(Duration::from_secs(1));

    let stdout = io::stdout();
    let mut display = Display::new(stdout);

    display.start();

    loop {
        sys.refresh_all();
        display.reset();

        let components = get_components_filtered(&sys, &nvidia, &args.components);

        let components_show = components_show(components);

        for cs in components_show {
            display.show(cs);
        }

        let stop = display.read_stop_event();

        if stop {
            break
        }

        thread::sleep(Duration::from_secs(args.update as u64));
    }

    display.stop();
}