mod device;
mod processor;
mod property;
mod sensor;

use crate::app::house::room::property::{
    Humidity, Light, Property, Radiation, Smoke, Sound, Temperature, Vibration,
};
use std::collections::HashMap;

pub struct Room {
    properties: HashMap<String, Box<dyn Property>>,
}

impl Room {
    pub fn new() -> Room {
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

        Room { properties }
    }
}
