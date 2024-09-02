use crate::app::house::room::property::Property;
use std::collections::HashMap;

pub trait Measurable {
    type Output;

    fn sense(&self) -> Self::Output;
}

//since they can only change see or change the value by the kye then it will need to receive a reference to the hashmap
pub struct TemperatureSensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}
impl TemperatureSensor {}
impl Measurable for TemperatureSensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct MovementSensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}

impl MovementSensor {}
impl Measurable for MovementSensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct LuminositySensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}
impl LuminositySensor {}
impl Measurable for LuminositySensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct RadiationSensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}
impl RadiationSensor {}
impl Measurable for RadiationSensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct HumiditySensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}
impl HumiditySensor {}
impl Measurable for HumiditySensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct SoundSensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}
impl SoundSensor {}
impl Measurable for SoundSensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
pub struct SmokeSensor {
    properties: &HashMap<String, Box<dyn Property>>,
    key: String,
}
impl SoundSensor {}
impl Measurable for SmokeSensor {
    type Output = u16;

    fn sense(&self) -> Self::Output {
        self.properties.get("Temperature").unwrap().get_value()
    }
}
