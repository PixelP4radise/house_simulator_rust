use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Sensor;
use std::rc::Weak;

pub struct InBetween {
    first_parameter: i16,
    second_parameter: i16,
    sensor: Weak<dyn Sensor>,
}

impl InBetween {}

impl Rule for InBetween {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() > self.first_parameter
            && self.sensor.upgrade().unwrap().sense() < self.second_parameter
    }
}
