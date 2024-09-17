use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Lamp {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: Option<String>,
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
                command: None,
                ticks_since_last_command: 0,
            }
        }
    }
}

impl DescribableItem for Lamp {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("d{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Lamp")
    }
}

impl Tickable for Lamp {
    fn tick(&mut self) {
        if let Some(command) = &self.command {
            match command.as_str() {
                "on" => {
                    let properties_rc = self.properties.upgrade().unwrap();
                    let mut properties = properties_rc.borrow_mut();

                    let light = properties.get_mut("Light").unwrap();

                    if self.ticks_since_last_command == 0 {
                        let current_value = light.get_value();
                        let new_value = current_value + 900;
                        light.set_value(new_value);
                    }
                }
                "off" => {
                    let properties_rc = self.properties.upgrade().unwrap();
                    let mut properties = properties_rc.borrow_mut();

                    let light = properties.get_mut("Light").unwrap();

                    if self.ticks_since_last_command == 0 {
                        let current_value = light.get_value();
                        let new_value = current_value - 900;
                        light.set_value(new_value);
                    }
                }
                _ => {}
            }
            self.ticks_since_last_command += 1;
        }
    }
}

impl Device for Lamp {
    fn ticks_since_last_command(&self) -> usize {
        self.ticks_since_last_command
    }

    fn command(&self) -> &Option<String> {
        &self.command
    }
    fn set_command(&mut self, command: String) {
        if let Some(old_command) = &self.command {
            match old_command.cmp(&command) {
                Ordering::Equal => {}
                _ => {
                    self.command = Some(command);
                    self.ticks_since_last_command = 0;
                }
            }
        }
    }
}
