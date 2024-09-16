use crate::app::house::room::processor::rule::{Rule, RULE_COUNTER};
use crate::app::house::room::sensor::Sensor;
use crate::app::house::DescribableItem;
use std::rc::Weak;

pub struct InBetween {
    first_parameter: i16,
    second_parameter: i16,
    sensor: Weak<dyn Sensor>,
    id: usize,
}

impl InBetween {
    pub fn new(first_parameter: i16, second_parameter: i16, sensor: Weak<dyn Sensor>) -> Self {
        unsafe {
            let id = RULE_COUNTER;
            RULE_COUNTER += 1;
            Self {
                first_parameter,
                second_parameter,
                sensor,
                id,
            }
        }
    }
}

impl DescribableItem for InBetween {
    fn id(&self) -> usize {
        self.id
    }

    fn full_id(&self) -> String {
        format!("r{}", self.id)
    }

    fn name(&self) -> String {
        String::from("In Between")
    }
}

impl Rule for InBetween {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() > self.first_parameter
            && self.sensor.upgrade().unwrap().sense() < self.second_parameter
    }
    fn get_sensor_name(&self) -> String {
        self.sensor.upgrade().unwrap().name()
    }

    fn get_sensor_full_id(&self) -> String {
        self.sensor.upgrade().unwrap().full_id()
    }
}

impl Clone for InBetween {
    fn clone(&self) -> Self {
        Self {
            first_parameter: self.first_parameter,
            second_parameter: self.second_parameter,
            sensor: self.sensor.clone(),
            id: self.id,
        }
    }
}
