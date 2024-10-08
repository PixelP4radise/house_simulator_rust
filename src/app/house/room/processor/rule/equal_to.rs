use crate::app::house::room::processor::rule::{Rule, RULE_COUNTER};
use crate::app::house::room::sensor::Sensor;
use crate::app::house::DescribableItem;
use std::rc::Weak;

pub struct EqualTo {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
    id: usize,
}

impl EqualTo {
    pub fn new(parameter: i16, sensor: Weak<dyn Sensor>) -> Self {
        unsafe {
            let id = RULE_COUNTER;
            RULE_COUNTER += 1;
            Self {
                parameter,
                sensor,
                id,
            }
        }
    }
}

impl DescribableItem for EqualTo {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("r{}", self.id)
    }

    fn name(&self) -> String {
        String::from("Equal To")
    }
}

impl Rule for EqualTo {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() == self.parameter
    }
    fn get_sensor_name(&self) -> String {
        self.sensor.upgrade().unwrap().name()
    }

    fn get_sensor_full_id(&self) -> String {
        self.sensor.upgrade().unwrap().full_id()
    }
}

impl Clone for EqualTo {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            parameter: self.parameter,
            sensor: self.sensor.clone(),
        }
    }
}
