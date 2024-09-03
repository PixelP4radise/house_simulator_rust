use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Sensor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct TemperatureSensor {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
}
impl TemperatureSensor {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        Self { properties }
    }
}
impl Sensor for TemperatureSensor {
    fn sense(&self) -> i16 {
        todo!()
    }
}
