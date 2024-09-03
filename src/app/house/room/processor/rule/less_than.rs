use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Sensor;
use std::rc::Weak;

pub struct LessThan {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
}

impl LessThan {}

impl Rule for LessThan {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() < self.parameter
    }
}
