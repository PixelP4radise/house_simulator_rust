use crate::app::house::room::processor::rule::{Rule, RULE_COUNTER};
use crate::app::house::room::sensor::Sensor;
use crate::app::house::DescribableItem;
use std::rc::Weak;

pub struct LessThan {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
    id: usize,
}

impl LessThan {
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

impl DescribableItem for LessThan {
    fn id(&self) -> usize {
        todo!()
    }

    fn full_id(&self) -> String {
        todo!()
    }

    fn name(&self) -> String {
        String::from("Less Than")
    }
}

impl Rule for LessThan {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() < self.parameter
    }
    fn get_sensor_name(&self) -> String {
        self.sensor.upgrade().unwrap().name()
    }

    fn get_sensor_full_id(&self) -> String {
        self.sensor.upgrade().unwrap().full_id()
    }
}

impl Clone for LessThan {
    fn clone(&self) -> Self {
        Self {
            parameter: self.parameter,
            id: self.id,
            sensor: self.sensor.clone(),
        }
    }
}
