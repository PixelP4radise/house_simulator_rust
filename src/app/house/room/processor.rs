mod rule;

use crate::app::house::{DescribableItem, Tickable};
use rule::{EqualTo, GreaterThan, InBetween, LessThan, Outside, Rule};

static mut PROCESSOR_COUNTER: usize = 0;

pub struct Processor {
    rules: Vec<Box<dyn Rule>>,
    id: usize,
    command: String,
}

impl Processor {
    pub fn new(command: String) -> Self {
        unsafe {
            let id = PROCESSOR_COUNTER;
            PROCESSOR_COUNTER += 1;
            Self {
                rules: vec![],
                id,
                command,
            }
        }
    }

    pub fn id(&self) -> String {
        let id: String = self.id.to_string();
        let id = format!("p{id}");
        id
    }

    pub fn rules_number(&self) -> usize {
        self.rules.len()
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
    fn tick(&self) {
        todo!()
    }
}
