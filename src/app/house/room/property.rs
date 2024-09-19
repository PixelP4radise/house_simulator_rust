pub trait Property {
    fn get_value(&self) -> i16;
    fn set_value(&mut self, value: i16) -> Result<(), &'static str>;
}

mod humidity;
mod light;
mod radiation;
mod smoke;
mod sound;
mod temperature;
mod vibration;

pub use self::{
    humidity::Humidity, light::Light, radiation::Radiation, smoke::Smoke, sound::Sound,
    temperature::Temperature, vibration::Vibration,
};
