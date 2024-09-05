use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::{Sensor, SENSOR_COUNTER};
use crate::app::house::DescribableItem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct TemperatureSensor {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
}
impl TemperatureSensor {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        unsafe {
            let id = SENSOR_COUNTER;
            SENSOR_COUNTER += 1;
            Self { properties, id }
        }
    }
}

impl DescribableItem for TemperatureSensor {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("s{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Temperature Sensor")
    }
}

impl Sensor for TemperatureSensor {
    fn sense(&self) -> i16 {
        self.properties
            .upgrade()
            .unwrap()
            .borrow()
            .get("Temperature")
            .unwrap()
            .get_value()
    }
}
