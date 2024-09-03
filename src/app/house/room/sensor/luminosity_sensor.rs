use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Measurable;
use std::collections::HashMap;

pub struct LuminositySensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> LuminositySensor<'a> {}
impl<'a> Measurable for LuminositySensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
