use crate::app::house::room::device::Device;
use crate::app::house::room::property::Property;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Cooler {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
}

impl Cooler {}

impl Device for Cooler {}
