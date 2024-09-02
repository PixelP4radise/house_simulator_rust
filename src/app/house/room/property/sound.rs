use crate::app::house::room::property::Property;

#[derive(Default)]
pub struct Sound {
    value: i16,
}
impl Property for Sound {
    fn get_value(&self) -> i16 {
        self.value
    }
}
