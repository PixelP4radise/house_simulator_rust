use crate::app::house::room::property::Property;
use std::collections::HashMap;

pub trait Measurable {
    fn sense(&self) -> i16;
}

//since they can only change see or change the value by the kye then it will need to receive a reference to the hashmap
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
pub struct LuminositySensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> LuminositySensor<'a> {}
impl<'a> Measurable for LuminositySensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct RadiationSensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> RadiationSensor<'a> {}
impl<'a> Measurable for RadiationSensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct HumiditySensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> HumiditySensor<'a> {}
impl<'a> Measurable for HumiditySensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct SoundSensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> SoundSensor<'a> {}
impl<'a> Measurable for SoundSensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct SmokeSensor<'a> {
    properties: &'a HashMap<String, Box<dyn Property>>,
    key: String,
}
impl<'a> SoundSensor<'a> {}
impl<'a> Measurable for SmokeSensor<'a> {
    fn sense(&self) -> i16 {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
