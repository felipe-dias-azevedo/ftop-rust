use sysinfo::{System, SystemExt};

pub struct SystemData {
    pub system: String,
    pub kernel: String,
    pub version: String,
    pub system_full: String,
    pub boot_time: u64,
    pub distro: String,
    pub hostname: String,
}

impl SystemData {
    pub fn new(sys: &System) -> SystemData {
        let kernel = sys.kernel_version().unwrap_or_default();
        let system = sys.name().unwrap_or_default();
        let version = sys.os_version().unwrap_or_default();
        let system_full = sys.long_os_version().unwrap_or_default();
        let boot_time = sys.boot_time();
        let distro = sys.distribution_id();
        let hostname = sys.host_name().unwrap_or_default();

        SystemData {
            system,
            kernel,
            version,
            system_full,
            boot_time,
            distro,
            hostname,
        }
    }
}