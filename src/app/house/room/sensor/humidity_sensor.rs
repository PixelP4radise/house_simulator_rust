use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Sensor;
use std::collections::HashMap;

pub struct HumiditySensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
}
impl<'a> HumiditySensor<'a> {
    pub fn new(properties: &'a HashMap<String, Box<dyn Property>>) -> Self {
        Self { properties }
    }
}
impl<'a> Sensor for HumiditySensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
