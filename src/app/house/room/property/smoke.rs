use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Smoke {
    value: i16,
}

impl Property for Smoke {
    fn get_value(&self) -> i16 {
        self.value
    }
}
