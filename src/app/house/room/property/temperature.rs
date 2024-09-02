use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Temperature {
    value: i16,
}

impl Property for Temperature {
    fn get_value(&self) -> i16 {
        self.value
    }
}
