use crate::app::house::room::sensor::Measurable;
use std::rc::Weak;

pub trait Rule {
    fn assert(&self) -> bool;
}

mod equal_to;
mod greater_than;
mod in_between;
mod less_than;
mod outside;
