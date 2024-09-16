use crate::app::house::room::device::{Device, DEVICE_COUNTER};
use crate::app::house::room::property::Property;
use crate::app::house::{DescribableItem, Tickable};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Weak;

pub struct Heater {
    properties: Weak<RefCell<HashMap<String, Box<dyn Property>>>>,
    id: usize,
    command: Option<String>,
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
                command: None,
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
    fn tick(&mut self) {
        if let Some(command) = &self.command {
            match command.as_str() {
                "on" => {
                    let properties_rc = self.properties.upgrade().unwrap();

                    let mut properties = properties_rc.borrow_mut();

                    if self.ticks_since_last_command % 3 == 0 {
                        let mut temperature = properties.get_mut("Temperature").unwrap();

                        let current_value = temperature.get_value();
                        let new_value = current_value + 1;

                        temperature.update_value(new_value.min(50));
                    }
                    if self.ticks_since_last_command == 0 {
                        let mut sound = properties.get_mut("Sound").unwrap();

                        let current_value = sound.get_value();
                        let new_value = current_value + 5;

                        sound.update_value(new_value);
                    }
                }
                "off" => {}
                _ => {}
            }
            self.ticks_since_last_command += 1;
        }
    }
}

impl Device for Heater {
    fn ticks_since_last_command(&self) -> usize {
        self.ticks_since_last_command
    }

    fn command(&self) -> &Option<String> {
        &self.command
    }
    fn set_command(&mut self, command: String) {
        //self.command = Some(command);
        todo!()
    }
}
