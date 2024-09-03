use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Sensor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct SoundSensor {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
}
impl SoundSensor {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        Self { properties }
    }
}
impl Sensor for SoundSensor {
    fn sense(&self) -> i16 {
        todo!()
    }
}
