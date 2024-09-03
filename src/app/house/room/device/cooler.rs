use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Cooler {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
}

impl Cooler {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        unsafe {
            let id = DEVICE_COUNTER;
            DEVICE_COUNTER += 1;
            Self { properties, id }
        }
    }
}

impl Device for Cooler {
    fn tick(&self) {
        todo!()
    }
}
