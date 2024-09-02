use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Measurable;
use std::rc::Weak;

pub struct GreaterThan {
    parameter: i16,
    sensor: Weak<dyn Measurable>,
}

impl GreaterThan {}

impl Rule for GreaterThan {
    fn assert(&self) -> bool {
        todo!()
    }
}
