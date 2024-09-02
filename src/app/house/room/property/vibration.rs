#[derive(Default)]
pub struct Vibration {
    value: i16,
}

impl Property for Vibration {
    fn get_value(&self) -> i16 {
        self.value
    }
}
