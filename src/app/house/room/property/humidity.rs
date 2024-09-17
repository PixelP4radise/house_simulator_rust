use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Humidity {
    value: i16,
}

impl Property for Humidity {
    fn get_value(&self) -> i16 {
        self.value
    }

    fn set_value(&mut self, value: i16) {
        self.value = value;
    }
}
