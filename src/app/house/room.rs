mod device;
mod processor;
mod property;
mod sensor;

use self::{
    device::Device,
    processor::Processor,
    property::{Humidity, Light, Property, Radiation, Smoke, Sound, Temperature, Vibration},
    sensor::{
        HumiditySensor, LuminositySensor, MovementSensor, RadiationSensor, Sensor, SmokeSensor,
        SoundSensor, TemperatureSensor,
    },
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

static mut ROOM_COUNTER: usize = 0;

pub struct Room {
    properties: Rc<RefCell<HashMap<String, Box<dyn Property>>>>,
    processors: Vec<Processor>,
    sensors: Vec<Box<dyn Sensor>>,
    devices: Vec<Box<dyn Device>>,
    id: usize,
    row: u8,
    column: u8,
}
//it's missing location on the house

impl Room {
    pub fn new(row: u8, column: u8) -> Self {
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

        unsafe {
            let id = ROOM_COUNTER;
            ROOM_COUNTER += 1;
            Self {
                properties: Rc::new(RefCell::new(properties)),
                processors: vec![],
                sensors: vec![],
                devices: vec![],
                id,
                row,
                column,
            }
        }
    }

    pub fn row(&self) -> u8 {
        self.row
    }

    pub fn column(&self) -> u8 {
        self.column
    }

    pub fn add_sensor(&mut self, sensor_type: &str) {
        match sensor_type {
            "humidity" => self
                .sensors
                .push(Box::new(HumiditySensor::new(Rc::downgrade(
                    &self.properties,
                )))),
            "luminosity" => self
                .sensors
                .push(Box::new(LuminositySensor::new(Rc::downgrade(
                    &self.properties,
                )))),
            "movement" => self
                .sensors
                .push(Box::new(MovementSensor::new(Rc::downgrade(
                    &self.properties,
                )))),
            "radiation" => self
                .sensors
                .push(Box::new(RadiationSensor::new(Rc::downgrade(
                    &self.properties,
                )))),
            "smoke" => self
                .sensors
                .push(Box::new(SmokeSensor::new(Rc::downgrade(&self.properties)))),
            "sound" => self
                .sensors
                .push(Box::new(SoundSensor::new(Rc::downgrade(&self.properties)))),
            "temperature" => self
                .sensors
                .push(Box::new(TemperatureSensor::new(Rc::downgrade(
                    &self.properties,
                )))),
            _ => {}
        }
    }

    pub fn change_property_value(&mut self, property: &str, value: i16) {
        self.properties
            .borrow_mut()
            .get_mut(property)
            .unwrap()
            .update_value(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn adding_a_sensor() {
        let mut room = Room::new();

        room.add_sensor("humidity");

        assert_eq!(1, room.sensors.len());
    }

    #[test]
    pub fn measuring_default_properties_values() {
        let mut room = Room::new();

        room.add_sensor("humidity");

        assert_eq!(
            0,
            room.properties
                .borrow()
                .get("Humidity")
                .unwrap()
                .get_value()
        );
    }

    #[test]
    pub fn measuring_changed_properties_values() {
        let mut room = Room::new();

        room.add_sensor("humidity");

        room.change_property_value("Humidity", 40);

        assert_eq!(
            40,
            room.properties
                .borrow()
                .get("Humidity")
                .unwrap()
                .get_value()
        );
    }
}
