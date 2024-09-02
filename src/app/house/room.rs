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
        Room {
            properties: HashMap::new(),
        }
    }
}
