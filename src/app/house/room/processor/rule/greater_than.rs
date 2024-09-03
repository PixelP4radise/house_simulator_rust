use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Sensor;
use std::rc::Weak;

pub struct GreaterThan {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
}

impl GreaterThan {}

impl Rule for GreaterThan {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() > self.parameter
    }
}
