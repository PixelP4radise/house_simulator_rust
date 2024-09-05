use crate::app::house::room::device::{Device, Sprinkler, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Lamp {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: String,
    ticks_since_last_command: usize,
}

impl Lamp {
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

impl DescribableItem for Lamp {
    fn id(&self) -> usize {
        todo!()
    }

    fn full_id(&self) -> String {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}

impl Tickable for Lamp {
    fn tick() {
        todo!()
    }
}

impl Device for Lamp {
    fn ticks_since_last_command() -> usize {
        todo!()
    }

    fn command() -> String {
        todo!()
    }
}
