mod rule;

use crate::app::house::room::device::Device;
use crate::app::house::room::sensor::Sensor;
use crate::app::house::{DescribableItem, Tickable};
use rule::{EqualTo, GreaterThan, InBetween, LessThan, Outside, Rule};
use std::cell::RefCell;
use std::rc::Weak;

static mut PROCESSOR_COUNTER: usize = 0;

pub enum ParameterNumber {
    One(i16),
    Two(i16, i16),
}

pub struct Processor {
    rules: Vec<Box<dyn Rule>>,
    id: usize,
    command: String,
    devices: Vec<Weak<RefCell<dyn Device>>>,
    room_id: String,
}

impl Processor {
    pub fn new(command: String, room_id: String) -> Self {
        unsafe {
            let id = PROCESSOR_COUNTER;
            PROCESSOR_COUNTER += 1;
            Self {
                rules: vec![],
                id,
                command,
                devices: vec![],
                room_id,
            }
        }
    }

    pub fn rules_number(&self) -> usize {
        self.rules.len()
    }

    pub fn add_rule(
        &mut self,
        rule_type: &str,
        sensor: Weak<dyn Sensor>,
        parameters: ParameterNumber,
    ) -> Result<(), &'static str> {
        let (param_1, param_2) = match parameters {
            ParameterNumber::One(p1) => (p1, None),
            ParameterNumber::Two(p1, p2) => (p1, Some(p2)),
        };

        match rule_type {
            "equal_to" => {
                if let Some(..) = param_2 {
                    Err("this rule only needs one parameter")
                } else {
                    Ok(self.rules.push(Box::new(EqualTo::new(param_1, sensor))))
                }
            }
            "greater_than" => {
                if let Some(..) = param_2 {
                    Err("this rule only needs one parameter")
                } else {
                    Ok(self.rules.push(Box::new(GreaterThan::new(param_1, sensor))))
                }
            }
            "less_than" => {
                if let Some(..) = param_2 {
                    Err("this rule only needs one parameter")
                } else {
                    Ok(self.rules.push(Box::new(LessThan::new(param_1, sensor))))
                }
            }
            "in_between" => {
                if let Some(param_2) = param_2 {
                    Ok(self
                        .rules
                        .push(Box::new(InBetween::new(param_1, param_2, sensor))))
                } else {
                    Err("this rule requires two parameters")
                }
            }
            "outside" => {
                if let Some(param_2) = param_2 {
                    Ok(self
                        .rules
                        .push(Box::new(Outside::new(param_1, param_2, sensor))))
                } else {
                    Err("this rule requires two parameters")
                }
            }
            _ => Err("the rule specified doesn't exist"),
        }
    }

    pub fn change_command(&mut self, command: String) {
        self.command = command;
    }

    pub fn list_rules(&self) -> String {
        self.rules
            .iter()
            .map(|rule| {
                format!(
                    "{} {} {} {}\n",
                    rule.name(),
                    rule.full_id(),
                    rule.name(),
                    rule.full_id()
                )
            })
            .collect::<String>()
    }

    pub fn remove_rule(&mut self, rule_id: &str) -> Result<(), &'static str> {
        let index = self.find_rule(rule_id)?;
        self.rules.remove(index);
        Ok(())
    }

    fn find_rule(&self, rule_id: &str) -> Result<usize, &'static str> {
        match self.rules.iter().position(|rule| rule.full_id() == rule_id) {
            Some(index) => Ok(index),
            None => Err("there was no rule with the specified id"),
        }
    }

    pub fn associate_device(&mut self, device: Weak<RefCell<dyn Device>>) {
        self.devices.push(device);
    }

    fn find_device(&self, device_id: &str) -> Result<usize, &'static str> {
        match self
            .devices
            .iter()
            .position(|device| device.upgrade().unwrap().borrow().full_id() == device_id)
        {
            Some(index) => Ok(index),
            None => Err("the device with the specified id couldn't be found"),
        }
    }

    pub fn remove_device_association(&mut self, device_id: &str) -> Result<(), &'static str> {
        let index = self.find_device(device_id)?;
        self.devices.remove(index);
        Ok(())
    }

    pub fn room_id(&self) -> &str {
        &self.room_id
    }
}

impl DescribableItem for Processor {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("p{}", self.id())
    }

    fn name(&self) -> String {
        String::from("Processor")
    }
}

impl Tickable for Processor {
    fn tick(&mut self) {
        if self.rules.len() == 0 {
            return;
        }
        for rule in &self.rules {
            if !rule.assert() {
                return;
            }
        }
        for device in &self.devices {
            device
                .upgrade()
                .unwrap()
                .borrow_mut()
                .set_command(self.command.clone())
        }
    }
}

impl Clone for Processor {
    fn clone(&self) -> Self {
        let rules: Vec<Box<dyn Rule>> = self.rules.iter().map(|rule| rule.clone_box()).collect();
        let devices = self.devices.iter().map(|device| device.clone()).collect();

        Processor {
            id: self.id,
            command: self.command.clone(),
            room_id: self.room_id.clone(),
            rules,
            devices,
        }
    }
}
