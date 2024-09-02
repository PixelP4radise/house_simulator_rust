use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Measurable;
use std::rc::Weak;

pub struct InBetween {
    first_parameter: i16,
    second_parameter: i16,
    sensor: Weak<dyn Measurable>,
}

impl InBetween {}

impl Rule for InBetween {
    fn assert(&self) -> bool {
        todo!()
    }
}
