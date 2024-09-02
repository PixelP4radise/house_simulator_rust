use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Measurable;
use std::rc::Weak;

pub struct LessThan {
    parameter: i16,
    sensor: Weak<dyn Measurable>,
}

impl LessThan {}

impl Rule for LessThan {
    fn assert(&self) -> bool {
        todo!()
    }
}
