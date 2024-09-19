use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Vibration {
    value: i16,
}

impl Vibration {
    const MIN: i16 = 0;
}

impl Property for Vibration {
    fn get_value(&self) -> i16 {
        self.value
    }
    fn set_value(&mut self, value: i16) -> Result<(), &'static str> {
        if value >= Self::MIN {
            Ok(self.value = value)
        } else {
            Err("The value for Vibration property must higher or equal to 0")
        }
    }
}
