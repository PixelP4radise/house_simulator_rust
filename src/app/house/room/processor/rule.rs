use crate::app::house::room::sensor::Sensor;

pub trait Rule {
    fn assert(&self) -> bool;
}

mod equal_to;
mod greater_than;
mod in_between;
mod less_than;
mod outside;

use equal_to::EqualTo;
use greater_than::GreaterThan;
use in_between::InBetween;
use less_than::LessThan;
use outside::Outside;
