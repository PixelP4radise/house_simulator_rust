mod device;
mod processor;
mod property;
mod sensor;

use self::sensor::{
    HumiditySensor, LuminositySensor, MovementSensor, RadiationSensor, Sensor, SmokeSensor,
    SoundSensor, TemperatureSensor,
};
use crate::app::house::room::device::Device;
use processor::Processor;
use property::{Humidity, Light, Property, Radiation, Smoke, Sound, Temperature, Vibration};
use std::collections::HashMap;
use std::slice::SplitN;

pub struct Room {
    properties: HashMap<String, Box<dyn Property>>,
    processors: Vec<Processor>,
    sensors: Vec<Box<dyn Sensor>>,
    devices: Vec<Box<dyn Device>>,
}
//it's missing location on the house

impl Room {
    pub fn new() -> Self {
        let mut properties: HashMap<String, Box<dyn Property>> = HashMap::new();

        properties.insert(
            String::from("Temperature"),
            Box::new(Temperature::default()),
        );
        properties.insert(String::from("Light"), Box::new(Light::default()));
        properties.insert(String::from("Radiation"), Box::new(Radiation::default()));
        properties.insert(String::from("Vibration"), Box::new(Vibration::default()));
        properties.insert(String::from("Humidity"), Box::new(Humidity::default()));
        properties.insert(String::from("Smoke"), Box::new(Smoke::default()));
        properties.insert(String::from("Sound"), Box::new(Sound::default()));

        Self {
            properties,
            processors: vec![],
            sensors: vec![],
            devices: vec![],
        }
    }

    pub fn add_sensor(&mut self, sensor_type: &str) {
        match sensor_type {
            "humidity" => self
                .sensors
                .push(Box::new(HumiditySensor::new(&self.properties))),
            "luminosity" => self
                .sensors
                .push(Box::new(LuminositySensor::new(&self.properties))),
            "movement" => self
                .sensors
                .push(Box::new(MovementSensor::new(&self.properties))),
            "radiation" => self
                .sensors
                .push(Box::new(RadiationSensor::new(&self.properties))),
            "smoke" => self
                .sensors
                .push(Box::new(SmokeSensor::new(&self.properties))),
            "sound" => self
                .sensors
                .push(Box::new(SoundSensor::new(&self.properties))),
            "temperature" => self
                .sensors
                .push(Box::new(TemperatureSensor::new(&self.properties))),
            _ => {}
        }
    }
}
