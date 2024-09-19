pub enum CurrentScreen {
    START,
    RUNNING,
    EXIT,
}

mod house;
use self::house::House;

pub struct App {
    house: Option<House>,
    current_screen: CurrentScreen,
    command: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            house: None,
            current_screen: CurrentScreen::START,
            command: String::new(),
        }
    }

    pub fn get_current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }
}
