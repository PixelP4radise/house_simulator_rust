pub trait Property {
    fn get_value(&self) -> i16;
    fn update_value(&mut self, value: i16);
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
