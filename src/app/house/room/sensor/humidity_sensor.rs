use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Measurable;
use std::collections::HashMap;

pub struct HumiditySensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> HumiditySensor<'a> {}
impl<'a> Measurable for HumiditySensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
