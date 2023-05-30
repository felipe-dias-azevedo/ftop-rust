mod monitor;
mod views;

use std::time::Duration;
use monitor::Component;
use nvml_wrapper::Nvml;
use sysinfo::{System, SystemExt};
use clap::Parser;

use crate::{monitor::{get_system_data, get_components_data}, views::displayutils::{start_message, list_components}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// List all components available
    #[arg(short, long, default_value_t = false)]
    list_components: bool,

    /// Components to monitoring
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
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

    println!("update: {}", args.update);
    println!("components: {:?}", args.components);
}