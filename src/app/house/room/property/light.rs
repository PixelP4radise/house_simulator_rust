use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Light {
    value: i16,
}

impl Light {
    const MIN: i16 = 0;
}

impl Property for Light {
    fn get_value(&self) -> i16 {
        self.value
    }
    fn set_value(&mut self, value: i16) -> Result<(), &'static str> {
        if value >= Self::MIN {
            Ok(self.value = value)
        } else {
            Err("The value for Light property must be higher or equal to 0")
        }
    }
}
