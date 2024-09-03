pub trait Device {}

mod cooler;
mod heater;
mod lamp;
mod sprinkler;

use self::{cooler::Cooler, heater::Heater, lamp::Lamp, sprinkler::Sprinkler};
