enum CurrentScreen {}

mod house;
use self::house::House;

pub struct App {
    house: Option<House>,
}

impl App {
    pub fn new() -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
