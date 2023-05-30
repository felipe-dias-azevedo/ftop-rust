use psutil::Error;
use psutil::sensors::{temperatures, TemperatureSensor};

use crate::views;

use super::{MonitorData, MonitorKind};

pub struct SensorData {
    name: String,
    label: Option<String>,
    temperature: f64,
    temperature_max: Option<f64>,
}

impl SensorData {
    fn get_temperatures() -> std::thread::Result<Vec<Result<TemperatureSensor, Error>>> {
        views::panicutils::catch_unwind_silent(|| {
            psutil::sensors::temperatures()
        })
    }

    pub fn new() -> Vec<SensorData> {
        let temperatures = Self::get_temperatures();

        if temperatures.is_err() {
            return vec![];
        }

        let temperatures = temperatures.unwrap();

        temperatures
            .into_iter()
            .filter_map(|t| t.ok())
            .map(|temp| {
                let unit = temp.unit();
                let temperature = temp.current().celsius().round();
                
                let temperature_max = match temp.high() {
                    Some(x) => Some(x.celsius().round()),
                    _ => None,
                };
                let label = match temp.label() {
                    Some(x) => Some(String::from(x)),
                    _ => None,
                };

                SensorData {
                    name: String::from(unit),
                    label,
                    temperature,
                    temperature_max,
                }
            })
            .collect()
    }

    pub fn format(sensors: Vec<SensorData>) -> MonitorData {
        let data = vec![];

        MonitorData {
            kind: MonitorKind::Sensors,
            data,
        }
    }
}