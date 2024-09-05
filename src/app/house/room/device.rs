pub trait Device: DescribableItem + Tickable {
    fn ticks_since_last_command(&self) -> usize;
    fn command(&self) -> String;
}

static mut DEVICE_COUNTER: usize = 0;

mod cooler;
mod heater;
mod lamp;
mod sprinkler;

pub use self::{cooler::Cooler, heater::Heater, lamp::Lamp, sprinkler::Sprinkler};
use crate::app::house::{DescribableItem, Tickable};
