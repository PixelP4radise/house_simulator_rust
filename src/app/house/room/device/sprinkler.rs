use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Sprinkler {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: Option<String>,
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
                command: None,
                ticks_since_last_command: 0,
            }
        }
    }

    fn remove_smoke(&self) {
        self.properties
            .upgrade()
            .unwrap()
            .borrow_mut()
            .get_mut("Smoke")
            .unwrap()
            .set_value(0)
            .unwrap();
    }

    fn increase_vibration(&self) {
        let properties_rc = self.properties.upgrade().unwrap();
        let mut properties = properties_rc.borrow_mut();

        let vibration = properties.get_mut("Vibration").unwrap();
        let current_value = vibration.get_value();
        let new_value = current_value + 100;

        vibration.set_value(new_value).unwrap();
    }

    fn decrease_vibration(&self) {
        let properties_rc = self.properties.upgrade().unwrap();
        let mut properties = properties_rc.borrow_mut();

        let vibration = properties.get_mut("Vibration").unwrap();

        let current_value = vibration.get_value();
        let new_value = current_value - 100;
        vibration.set_value(new_value).unwrap();
    }

    fn increase_humidity(&self) {
        let properties_rc = self.properties.upgrade().unwrap();

        let mut properties = properties_rc.borrow_mut();

        let humidity = properties.get_mut("Humidity").unwrap();
        let current_value = humidity.get_value();
        let new_value = current_value + 50;
        humidity.set_value(new_value.min(75)).unwrap();
    }
}

impl DescribableItem for Sprinkler {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("d{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Sprinkler")
    }
}

impl Tickable for Sprinkler {
    fn tick(&mut self) {
        if let Some(command) = &self.command {
            match command.as_str() {
                "on" => match self.ticks_since_last_command {
                    0 => {
                        self.increase_humidity();
                        self.increase_vibration();
                    }
                    1 => {
                        self.remove_smoke();
                    }
                    _ => {}
                },
                "off" => {
                    if self.ticks_since_last_command == 5 {
                        self.decrease_vibration();
                    }
                }
                _ => {}
            }
            self.ticks_since_last_command += 1;
        }
    }
}

impl Device for Sprinkler {
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
