mod rule;

use rule::{Rule, EqualTo, GreaterThan, InBetween, LessThan, Outside};

pub struct Processor {
    rules: Vec<Box<dyn Rule>>,
}

impl Processor {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }

    pub fn tick(&self) {}
}
