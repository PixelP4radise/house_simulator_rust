mod rule;

use rule::{EqualTo, GreaterThan, InBetween, LessThan, Outside, Rule};

static mut PROCESSOR_COUNTER: usize = 0;

pub struct Processor {
    rules: Vec<Box<dyn Rule>>,
    id: usize,
}

impl Processor {
    pub fn new() -> Self {
        unsafe {
            let id = PROCESSOR_COUNTER;
            PROCESSOR_COUNTER += 1;
            Self { rules: vec![], id }
        }
    }

    pub fn tick(&self) {}
}
