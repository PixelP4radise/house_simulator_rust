use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Temperature {
    value: i16,
}

impl Temperature {
    const MIN: i16 = -273;
}

impl Property for Temperature {
    fn get_value(&self) -> i16 {
        self.value
    }
    fn set_value(&mut self, value: i16) -> Result<(), &'static str> {
        if value >= Self::MIN {
            Ok(self.value = value)
        } else {
            Err("The value for Temperature property must be higher or equal to -273")
        }
    }
}
