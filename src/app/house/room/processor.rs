mod rule;

use rule::{EqualTo, GreaterThan, InBetween, LessThan, Outside, Rule};

pub struct Processor {
    rules: Vec<Box<dyn Rule>>,
}

impl Processor {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn tick(&self) {}
}
