mod rule;

use rule::Rule;

pub struct Processor {
    rules: Vec<Box<dyn Rule>>,
}

impl Processor {
    pub fn new() -> Self {
        Self { rules: vec![] }
    }
}
