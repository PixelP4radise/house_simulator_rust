use crate::app::house::room::processor::rule::{Rule, RULE_COUNTER};
use crate::app::house::room::sensor::Sensor;
use std::rc::Weak;

pub struct InBetween {
    first_parameter: i16,
    second_parameter: i16,
    sensor: Weak<dyn Sensor>,
    id: usize,
}

impl InBetween {
    pub fn new(first_parameter: i16, second_parameter: i16, sensor: Weak<dyn Sensor>) -> Self {
        unsafe {
            let id = RULE_COUNTER;
            RULE_COUNTER += 1;
            Self {
                first_parameter,
                second_parameter,
                sensor,
                id,
            }
        }
    }
}

impl Rule for InBetween {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() > self.first_parameter
            && self.sensor.upgrade().unwrap().sense() < self.second_parameter
    }
}
