enum CurrentScreen {}

mod house;
use crate::app::house::House;

pub struct App {
    house: House,
}

impl App {
    pub fn new() -> App {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
