mod device;
mod processor;
mod property;
mod sensor;

use self::{
    device::{Cooler, Device, Heater, Lamp, Sprinkler},
    property::{Humidity, Light, Property, Radiation, Smoke, Sound, Temperature, Vibration},
    sensor::{
        HumiditySensor, LuminositySensor, MovementSensor, RadiationSensor, Sensor, SmokeSensor,
        SoundSensor, TemperatureSensor,
    },
};

pub use self::processor::Processor;
use super::{DescribableItem, Tickable};
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
    y_coordinate: u8,
    x_coordinate: u8,
}
//it's missing location on the house

impl Room {
    pub fn new(x_coordinate: u8, y_coordinate: u8) -> Self {
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
                x_coordinate,
                y_coordinate,
            }
        }
    }

    pub fn y_coordinate(&self) -> u8 {
        self.y_coordinate
    }

    pub fn x_coordinate(&self) -> u8 {
        self.x_coordinate
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
            Some(property) => Ok(property.set_value(value)?),
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
        self.processors
            .push(Processor::new(command, self.full_id()));
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
                let command_str = match device.command() {
                    Some(command) => command.clone(), // Clone the command string
                    None => String::new(),            // Default to an empty string
                };
                format!("{} {} {}\n", device.full_id(), device.name(), command_str)
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

    pub fn change_command_from_processor(
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

    pub fn copy_processor(&self, processor_id: &str) -> Result<Processor, &'static str> {
        let index = self.find_processor(processor_id)?;
        Ok(self.processors[index].clone())
    }

    pub fn restore_processor(&mut self, processor: Processor) {
        if let Ok(index) = self.find_processor(processor.full_id().as_str()) {
            self.processors[index] = processor;
        } else {
            self.processors.push(processor);
        }
    }

    pub fn get_description(&self) -> String {
        format!(
            "ID: {}\nProcessors:\n{}\nDevices:\n{}\nSensors:\n{}",
            self.full_id(),
            self.processors
                .iter()
                .map(|processor| processor.full_id())
                .collect::<String>(),
            self.devices
                .iter()
                .map(|device| device.borrow().full_id())
                .collect::<String>(),
            self.sensors
                .iter()
                .map(|sensor| sensor.full_id())
                .collect::<String>()
        )
    }

    pub fn change_command_from_device(
        &mut self,
        device_id: &str,
        command: String,
    ) -> Result<(), &'static str> {
        let index = self.find_device(device_id)?;

        self.devices[index].borrow_mut().set_command(command);
        Ok(())
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

impl Tickable for Room {
    fn tick(&mut self) {
        for processor in &mut self.processors {
            processor.tick();
        }
        for device in &mut self.devices {
            device.borrow_mut().tick();
        }
    }
}
