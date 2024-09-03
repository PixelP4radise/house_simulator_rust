use std::collections::HashMap;

pub struct MovementSensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}

impl<'a> MovementSensor<'a> {}
impl<'a> Measurable for MovementSensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
