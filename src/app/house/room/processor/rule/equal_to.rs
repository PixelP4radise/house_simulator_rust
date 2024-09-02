use crate::app::house::room::processor::rule::Rule;
use crate::app::house::room::sensor::Measurable;
use std::rc::Weak;

pub struct EqualTo {
    parameter: i16,
    sensor: Weak<dyn Measurable>,
}

impl EqualTo {}

impl Rule for EqualTo {
    fn assert(&self) -> bool {
        todo!()
    }
}
