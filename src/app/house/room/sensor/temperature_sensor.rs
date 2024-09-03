use std::collections::HashMap;

pub struct TemperatureSensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> TemperatureSensor<'a> {}
impl<'a> Measurable for TemperatureSensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
