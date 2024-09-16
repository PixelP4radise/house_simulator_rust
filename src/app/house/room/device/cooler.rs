use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Cooler {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: Option<String>,
    ticks_since_last_command: usize,
}

impl Cooler {
    pub fn new(properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>) -> Self {
        unsafe {
            let id = DEVICE_COUNTER;
            DEVICE_COUNTER += 1;
            Self {
                properties,
                id,
                command: None,
                ticks_since_last_command: 0,
            }
        }
    }
}

impl DescribableItem for Cooler {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("d{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Cooler")
    }
}

impl Tickable for Cooler {
    fn tick(&self) {
        todo!()
    }
}

impl Device for Cooler {
    fn ticks_since_last_command(&self) -> usize {
        self.ticks_since_last_command
    }

    fn command(&self) -> &Option<String> {
        &self.command
    }
    fn set_command(&mut self, command: String) {
        self.command = Some(command);
    }
}
