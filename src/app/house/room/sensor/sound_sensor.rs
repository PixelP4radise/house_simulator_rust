use crate::app::house::room::property::Property;
use crate::app::house::room::sensor::{Sensor, SENSOR_COUNTER};
use crate::app::house::DescribableItem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct SoundSensor {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
}
impl SoundSensor {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        unsafe {
            let id = SENSOR_COUNTER;
            SENSOR_COUNTER += 1;
            Self { properties, id }
        }
    }
}

impl DescribableItem for SoundSensor {
    fn id(&self) -> usize {
        todo!()
    }

    fn full_id(&self) -> String {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}

impl Sensor for SoundSensor {
    fn sense(&self) -> i16 {
        self.properties
            .upgrade()
            .unwrap()
            .borrow()
            .get("Sound")
            .unwrap()
            .get_value()
    }
}
