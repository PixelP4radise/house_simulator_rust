use crate::app::house::DescribableItem;

pub trait Rule: DescribableItem + RuleClone {
    fn assert(&self) -> bool;

    fn get_sensor_name(&self) -> String;

    fn get_sensor_full_id(&self) -> String;
}

pub trait RuleClone {
    fn clone_box(&self) -> Box<dyn Rule>;
}

impl<T> RuleClone for T
where
    T: 'static + Rule + Clone,
{
    fn clone_box(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Rule> {
    fn clone(&self) -> Box<dyn Rule> {
        self.clone_box()
    }
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
