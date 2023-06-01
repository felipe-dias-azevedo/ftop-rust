mod cpu;
mod gpu;
mod sensors;
mod ram;
mod network;
mod system;
mod disk;

use nvml_wrapper::{Nvml, error::NvmlError};
use strum_macros::{EnumString, Display};
use sysinfo::System;

use self::system::SystemData;

#[derive(Clone)]
pub struct MonitorData {
    pub kind: MonitorKind,
    pub data: Vec<Component>,
}

#[derive(Clone)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub data: String,
}

#[derive(Clone, Eq, PartialEq, EnumString, Display)]
pub enum MonitorKind {
    Cpu,
    Ram,
    Disk,
    Network,
    Sensors,
    Gpu,
}

pub fn get_components_data(sys: &System, nvidia: &Result<Nvml, NvmlError>) -> Vec<MonitorData> {
    let cpu = cpu::CpuData::new(sys).format();
    let ram = ram::RamData::new(sys).format();
    let disk = disk::DisksData::new(sys).format();
    let network = network::NetworkData::format(network::NetworkData::new(sys));
    let gpu = gpu::GpuData::format(gpu::GpuData::new(nvidia));
    let sensors = sensors::SensorData::format(sensors::SensorData::new());

    vec![cpu, ram, disk, network, sensors, gpu]
}

pub fn get_components_filtered(sys: &System, nvidia: &Result<Nvml, NvmlError>, filter: &Vec<String>) -> Vec<Component> {
    let components = get_components_data(&sys, &nvidia);

    let components = components.into_iter()
        .map(|md| md.data).collect::<Vec<Vec<Component>>>().concat();

    components.into_iter().filter(|components| {
        filter.contains(&components.id) ||
            filter.iter().filter(|f| {
                f.contains("*") && components.id.starts_with(&f.replace("*", ""))
            }).count() > 0
    }).collect()
}

pub fn get_system_data(sys: &System) -> SystemData {
    system::SystemData::new(sys)
}