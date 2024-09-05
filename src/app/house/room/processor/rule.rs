use crate::app::house::room::sensor::Sensor;
use crate::app::house::DescribableItem;

pub trait Rule: DescribableItem {
    fn assert(&self) -> bool;
}

static mut RULE_COUNTER: usize = 0;

mod equal_to;
mod greater_than;
mod in_between;
mod less_than;
mod outside;

pub use self::{
    equal_to::EqualTo, greater_than::GreaterThan, in_between::InBetween, less_than::LessThan,
    outside::Outside,
};
