use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Humidity {
    value: i16,
}

impl Humidity {
    const MIN: i16 = 0;
    const MAX: i16 = 100;
}

impl Property for Humidity {
    fn get_value(&self) -> i16 {
        self.value
    }

    fn set_value(&mut self, value: i16) -> Result<(), &'static str> {
        if value >= Self::MIN && value <= Self::MAX {
            Ok(self.value = value)
        } else {
            Err("The value for humidity property must be between 0 and 100")
        }
    }
}
