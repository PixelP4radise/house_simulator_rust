use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::Sensor;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct HumiditySensor {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
}
impl HumiditySensor {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        Self { properties }
    }
}
impl Sensor for HumiditySensor {
    fn sense(&self) -> i16 {
        self.properties
            .upgrade()
            .unwrap()
            .borrow()
            .get("Humidity")
            .unwrap()
            .get_value()
    }
}
