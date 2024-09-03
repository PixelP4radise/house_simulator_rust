use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Measurable;
use std::collections::HashMap;

pub struct RadiationSensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> RadiationSensor<'a> {}
impl<'a> Measurable for RadiationSensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
