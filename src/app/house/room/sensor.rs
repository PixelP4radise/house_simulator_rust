use crate::app::house::room::property::Property;
use crate::app::house::DescribableItem;

pub trait Sensor: DescribableItem {
    fn sense(&self) -> i16;
}

static mut SENSOR_COUNTER: usize = 0;

//since they can only change see or change the value by the kye then it will need to receive a reference to the hashmap
mod humidity_sensor;
mod luminosity_sensor;
mod movement_sensor;
mod radiation_sensor;
mod smoke_sensor;
mod sound_sensor;
mod temperature_sensor;

pub use self::{
    humidity_sensor::HumiditySensor, luminosity_sensor::LuminositySensor,
    movement_sensor::MovementSensor, radiation_sensor::RadiationSensor, smoke_sensor::SmokeSensor,
    sound_sensor::SoundSensor, temperature_sensor::TemperatureSensor,
};
