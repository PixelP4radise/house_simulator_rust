use crate::app::house::room::sensor::Measurable;
use std::rc::Weak;

pub trait Rule {
    fn assert(&self) -> bool;
}

mod equal_to;
mod less_than;

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

pub struct Outside {
    first_parameter: i16,
    second_parameter: i16,
    sensor: Weak<dyn Measurable>,
}

impl Outside {}

impl Rule for Outside {
    fn assert(&self) -> bool {
        todo!()
    }
}
