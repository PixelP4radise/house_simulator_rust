use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Smoke {
    value: i16,
}

impl Smoke {
    const MIN: i16 = 0;
    const MAX: i16 = 100;
}

impl Property for Smoke {
    fn get_value(&self) -> i16 {
        self.value
    }
    fn set_value(&mut self, value: i16) -> Result<(), &'static str> {
        if value >= Self::MIN && value <= Self::MAX {
            Ok(self.value = value)
        } else {
            Err("The value for Smoke property must be between 0 and 100")
        }
    }
}
