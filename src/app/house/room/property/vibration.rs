use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Vibration {
    value: i16,
}

impl Property for Vibration {
    fn get_value(&self) -> i16 {
        self.value
    }
    fn update_value(&mut self, value: i16) {
        self.value = value;
    }
}
