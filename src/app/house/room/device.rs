pub trait Device {
    fn tick(&self);
}

static mut DEVICE_COUNTER: usize = 0;

mod cooler;
mod heater;
mod lamp;
mod sprinkler;

pub use self::{cooler::Cooler, heater::Heater, lamp::Lamp, sprinkler::Sprinkler};
