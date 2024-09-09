use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Heater {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: String,
    ticks_since_last_command: usize,
}
impl Heater {
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

impl DescribableItem for Heater {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("d{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Heater")
    }
}

impl Tickable for Heater {
    fn tick(&self) {
        todo!()
    }
}

impl Device for Heater {
    fn ticks_since_last_command(&self) -> usize {
        todo!()
    }

    fn command(&self) -> String {
        todo!()
    }
    fn set_command(&mut self, command: String) {
        self.command = command;
    }
}
