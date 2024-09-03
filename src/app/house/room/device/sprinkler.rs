use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Sprinkler {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: String,
    ticks_since_last_command: usize,
}

impl Sprinkler {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        unsafe {
            let id = DEVICE_COUNTER;
            DEVICE_COUNTER += 1;
            Self {
                properties,
                id,
                command: String::new(),
                ticks_since_last_command: 0,
            }
        }
    }
}

impl Device for Sprinkler {
    fn tick(&self) {
        match self.command.as_str() {
            "on" => {}
            "off" => {}
            _ => {}
        }
    }
}
