use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::cmp::Ordering;
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

    fn activate_noise(&self) {
        let properties_rc = self.properties.upgrade().unwrap();

        let mut properties = properties_rc.borrow_mut();

        let sound = properties.get_mut("Sound").unwrap();

        let current_value = sound.get_value();
        let new_value = current_value + 20;
        sound.set_value(new_value).unwrap();
    }

    fn decrease_temperature(&self) {
        let properties_rc = self.properties.upgrade().unwrap();

        let mut properties = properties_rc.borrow_mut();

        let temperature = properties.get_mut("Temperature").unwrap();

        let current_value = temperature.get_value();
        let new_value = current_value - 1;

        temperature.set_value(new_value).unwrap();
    }

    fn deactivate_noise(&self) {
        let properties_rc = self.properties.upgrade().unwrap();

        let mut properties = properties_rc.borrow_mut();

        let sound = properties.get_mut("Sound").unwrap();

        let current_value = sound.get_value();
        let new_value = current_value - 20;
        sound.set_value(new_value).unwrap();
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
    fn tick(&mut self) {
        if let Some(command) = &self.command {
            match command.as_str() {
                "on" => {
                    if self.ticks_since_last_command == 0 {
                        self.activate_noise();
                    }

                    if (self.ticks_since_last_command + 1) % 3 == 0 {
                        self.decrease_temperature();
                    }
                }
                "off" => {
                    if self.ticks_since_last_command == 0 {
                        self.deactivate_noise();
                    }
                }
                _ => {}
            }
            self.ticks_since_last_command += 1;
        }
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
        if let Some(old_command) = &self.command {
            match old_command.cmp(&command) {
                Ordering::Equal => {}
                _ => {
                    self.command = Some(command);
                    self.ticks_since_last_command = 0;
                }
            }
        } else {
            self.command = Some(command);
        }
    }
}
