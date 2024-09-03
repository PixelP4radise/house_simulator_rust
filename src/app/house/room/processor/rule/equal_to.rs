use crate::app::house::room::processor::rule::{Rule, RULE_COUNTER};
use crate::app::house::room::sensor::Sensor;
use std::rc::Weak;

pub struct EqualTo {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
    id: usize,
}

impl EqualTo {
    pub fn new(parameter: i16, sensor: Weak<dyn Sensor>) -> Self {
        unsafe {
            let id = RULE_COUNTER;
            RULE_COUNTER += 1;
            Self {
                parameter,
                sensor,
                id,
            }
        }
    }
}

impl Rule for EqualTo {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() == self.parameter
    }
}
