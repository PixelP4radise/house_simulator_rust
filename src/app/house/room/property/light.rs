use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Light {
    value: i16,
}

impl Property for Light {
    fn get_value(&self) -> i16 {
        self.value
    }
}
