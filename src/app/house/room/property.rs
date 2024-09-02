pub trait Property {
    fn get_value(&self) -> i16;
}

mod humidity;
mod light;
mod radiation;
mod smoke;
mod sound;
mod temperature;
mod vibration;
