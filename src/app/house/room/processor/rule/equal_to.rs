use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Sensor;
use std::rc::Weak;

pub struct EqualTo {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
}

impl EqualTo {}

impl Rule for EqualTo {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() == self.parameter
    }
}
