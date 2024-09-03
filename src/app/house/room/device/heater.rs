use crate::app::house::room::device::Device;
use crate::app::house::room::property::Property;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Heater {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
}
impl Heater {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        Self { properties }
    }
}
impl Device for Heater {}
