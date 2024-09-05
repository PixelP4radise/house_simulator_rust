use crate::app::house::room::processor::rule::{Rule, RULE_COUNTER};
use crate::app::house::room::sensor::Sensor;
use crate::app::house::DescribableItem;
use std::rc::Weak;

pub struct GreaterThan {
    parameter: i16,
    sensor: Weak<dyn Sensor>,
    id: usize,
}

impl GreaterThan {
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

impl DescribableItem for GreaterThan {
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

impl Rule for GreaterThan {
    fn assert(&self) -> bool {
        self.sensor.upgrade().unwrap().sense() > self.parameter
    }
}
