use psutil::Error;
use psutil::sensors::{temperatures, TemperatureSensor};
use crate::monitor::Component;

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
            temperatures()
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
        let data = sensors.into_iter().map(|sensor| {
            let temperature = Component {
                id: match &sensor.label {
                    Some(label) => format!("{}-{}-temp", sensor.name, label.to_lowercase()),
                    _ => format!("{}-temp", sensor.name)
                },
                name: match &sensor.label {
                    Some(label) => format!("Sensor {} {} Temperature", sensor.name, label),
                    _ => format!("Sensor {} Temperature", sensor.name)
                } ,
                data: format!("{} ºC", sensor.temperature),
            };

            match sensor.temperature_max {
                Some(temp_max) => {
                    let temperature_max = Component {
                        id: match &sensor.label {
                            Some(label) => format!("{}-{}-temp-max", sensor.name, label.to_lowercase()),
                            _ => format!("{}-temp-max", sensor.name)
                        },
                        name: match &sensor.label {
                            Some(label) => format!("Sensor {} {} Temperature Max", sensor.name, label),
                            _ => format!("Sensor {} Temperature Max", sensor.name)
                        } ,
                        data: format!("{} ºC", temp_max),
                    };

                    vec![temperature, temperature_max]
                }
                _ => vec![temperature]
            }
        }).collect::<Vec<Vec<Component>>>().concat();

        MonitorData {
            kind: MonitorKind::Sensors,
            data,
        }
    }
}