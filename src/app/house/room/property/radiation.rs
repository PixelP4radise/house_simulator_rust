use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Radiation {
    value: i16,
}

impl Property for Radiation {
    fn get_value(&self) -> i16 {
        self.value
    }
}
