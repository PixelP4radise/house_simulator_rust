mod device;
mod processor;
mod property;
mod sensor;

use self::{
    device::{Cooler, Device, Heater, Lamp, Sprinkler},
    processor::Processor,
    property::{Humidity, Light, Property, Radiation, Smoke, Sound, Temperature, Vibration},
    sensor::{
        HumiditySensor, LuminositySensor, MovementSensor, RadiationSensor, Sensor, SmokeSensor,
        SoundSensor, TemperatureSensor,
    },
};
use super::DescribableItem;
pub use crate::app::house::room::processor::ParameterNumber;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

static mut ROOM_COUNTER: usize = 0;

pub struct Room {
    properties: Rc<RefCell<HashMap<String, Box<dyn Property>>>>,
    processors: Vec<Processor>,
    sensors: Vec<Rc<dyn Sensor>>,
    devices: Vec<Rc<RefCell<dyn Device>>>,
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

    pub fn sensors_number(&self) -> usize {
        self.sensors.len()
    }

    pub fn processors_number(&self) -> usize {
        self.processors.len()
    }

    pub fn devices_number(&self) -> usize {
        self.devices.len()
    }

    pub fn add_sensor(&mut self, sensor_type: &str) -> Result<(), &'static str> {
        match sensor_type {
            "humidity" => Ok(self.sensors.push(Rc::new(HumiditySensor::new(Rc::downgrade(
                &self.properties,
            ))))),
            "luminosity" => Ok(self
                .sensors
                .push(Rc::new(LuminositySensor::new(Rc::downgrade(
                    &self.properties,
                ))))),
            "movement" => Ok(self.sensors.push(Rc::new(MovementSensor::new(Rc::downgrade(
                &self.properties,
            ))))),
            "radiation" => Ok(self
                .sensors
                .push(Rc::new(RadiationSensor::new(Rc::downgrade(
                    &self.properties,
                ))))),
            "smoke" => Ok(self
                .sensors
                .push(Rc::new(SmokeSensor::new(Rc::downgrade(&self.properties))))),
            "sound" => Ok(self
                .sensors
                .push(Rc::new(SoundSensor::new(Rc::downgrade(&self.properties))))),
            "temperature" => Ok(self
                .sensors
                .push(Rc::new(TemperatureSensor::new(Rc::downgrade(
                    &self.properties,
                ))))),
            _ => Err("the sensor type specified doesn't exist"),
        }
    }

    pub fn change_property_value(
        &mut self,
        property: &str,
        value: i16,
    ) -> Result<(), &'static str> {
        match self.properties.borrow_mut().get_mut(property) {
            Some(property) => Ok(property.update_value(value)),
            None => Err("the specified property doesn't exist"),
        }
    }

    pub fn list_properties(&self) -> String {
        let mut properties_list: Vec<String> = self
            .properties
            .borrow()
            .iter()
            .map(|(key, value)| format!("{key}: {}\n", value.get_value()))
            .collect();

        properties_list.sort();
        properties_list.concat()
    }

    pub fn add_device(&mut self, device_type: &str) -> Result<(), &'static str> {
        match device_type {
            "cooler" => Ok(self
                .devices
                .push(Rc::new(RefCell::new(Cooler::new(Rc::downgrade(
                    &self.properties,
                )))))),
            "heater" => Ok(self
                .devices
                .push(Rc::new(RefCell::new(Heater::new(Rc::downgrade(
                    &self.properties,
                )))))),
            "lamp" => Ok(self
                .devices
                .push(Rc::new(RefCell::new(Lamp::new(Rc::downgrade(
                    &self.properties,
                )))))),
            "sprinkler" => {
                Ok(self
                    .devices
                    .push(Rc::new(RefCell::new(Sprinkler::new(Rc::downgrade(
                        &self.properties,
                    ))))))
            }
            _ => Err("device type not recognized"),
        }
    }

    pub fn add_processor(&mut self, command: String) {
        self.processors.push(Processor::new(command));
    }

    pub fn list_components(&self) -> String {
        self.processors
            .iter()
            .map(|processor| {
                format!(
                    "{} {} {}\n",
                    processor.full_id(),
                    processor.name(),
                    processor.rules_number()
                )
            })
            .chain(self.sensors.iter().map(|sensor| {
                format!(
                    "{} {} {}\n",
                    sensor.full_id(),
                    sensor.name(),
                    sensor.sense()
                )
            }))
            .chain(self.devices.iter().map(|device| {
                let device = device.borrow();
                format!(
                    "{} {} {}\n",
                    device.full_id(),
                    device.name(),
                    device.command()
                )
            }))
            .collect::<String>()
    }

    pub fn remove_device(&mut self, device_id: &str) -> Result<(), &'static str> {
        let index = self.find_device(device_id)?;
        if Rc::weak_count(&self.devices[index]) > 0 {
            return Err("Device can't be deleted while there are processors that reference it");
        }
        self.devices.remove(index);
        Ok(())
    }

    fn find_device(&self, device_id: &str) -> Result<usize, &'static str> {
        match self
            .devices
            .iter()
            .position(|device| device.borrow().full_id() == device_id)
        {
            Some(index) => Ok(index),
            None => Err("the device with the specified id couldn't be found"),
        }
    }

    pub fn remove_processor(&mut self, processor_id: &str) -> Result<(), &'static str> {
        let index = self.find_processor(processor_id)?;
        self.processors.remove(index);
        Ok(())
    }

    fn find_processor(&self, processor_id: &str) -> Result<usize, &'static str> {
        match self
            .processors
            .iter()
            .position(|processor| processor.full_id() == processor_id)
        {
            Some(index) => Ok(index),
            None => Err("the processor with the specified id couldn't be found"),
        }
    }

    pub fn remove_sensor(&mut self, sensor_id: &str) -> Result<(), &'static str> {
        let index = self.find_sensor(sensor_id)?;
        if Rc::weak_count(&self.sensors[index]) > 0 {
            return Err("Sensor can't be deleted while there are rules that depend on it");
        }
        self.sensors.remove(index);
        Ok(())
    }

    fn find_sensor(&mut self, sensor_id: &str) -> Result<usize, &'static str> {
        match self
            .sensors
            .iter()
            .position(|sensor| sensor.full_id() == sensor_id)
        {
            Some(index) => Ok(index),
            None => Err("the sensor with the specified id couldn't be found"),
        }
    }

    pub fn add_rule(
        &mut self,
        processor_id: &str,
        rule_type: &str,
        sensor_id: &str,
        parameters: ParameterNumber,
    ) -> Result<(), &'static str> {
        let processor_index = self.find_processor(processor_id)?;
        let sensor_index = self.find_sensor(sensor_id)?;

        self.processors[processor_index].add_rule(
            rule_type,
            Rc::downgrade(&self.sensors[sensor_index]),
            parameters,
        )
    }

    pub fn change_command(
        &mut self,
        processor_id: &str,
        command: String,
    ) -> Result<(), &'static str> {
        let index = self.find_processor(processor_id)?;
        Ok(self.processors[index].change_command(command))
    }

    pub fn list_rules(&self, processor_id: &str) -> Result<String, &'static str> {
        let index = self.find_processor(processor_id)?;
        Ok(self.processors[index].list_rules())
    }

    pub fn remove_rule(&mut self, processor_id: &str, rule_id: &str) -> Result<(), &'static str> {
        let index = self.find_processor(processor_id)?;
        self.processors[index].remove_rule(rule_id)
    }

    pub fn associate_device(
        &mut self,
        processor_id: &str,
        device_id: &str,
    ) -> Result<(), &'static str> {
        let processor_index = self.find_processor(processor_id)?;
        let device_index = self.find_device(device_id)?;
        Ok(self.processors[processor_index]
            .associate_device(Rc::downgrade(&self.devices[device_index])))
    }

    pub fn remove_device_association(
        &mut self,
        processor_id: &str,
        device_id: &str,
    ) -> Result<(), &'static str> {
        let index = self.find_processor(processor_id)?;
        Ok(self.processors[index].remove_device_association(device_id)?)
    }
}

impl DescribableItem for Room {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("r{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Room")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn adding_a_sensor() {
        let mut room = Room::new(1, 1);

        room.add_sensor("humidity");

        assert_eq!(1, room.sensors.len());
    }

    #[test]
    pub fn measuring_default_properties_values() {
        let mut room = Room::new(1, 1);

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
        let mut room = Room::new(1, 1);

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
