pub trait Property {
    type Output;
    fn get_value(&self) -> Self::Output;
    fn get_descrition(&self) -> &str;
}

//os sensores e aparelhos devem comunicar com as propriedades através das suas chaves

#[derive(Default)]
pub struct Temperature {
    value: u16,
}

impl Property for Temperature {}

#[derive(Default)]
pub struct Light {
    value: u16,
}

impl Property for Light {}

#[derive(Default)]
pub struct Radiation {
    value: u16,
}

impl Property for Radiation {}

#[derive(Default)]
pub struct Vibration {
    value: u16,
}

impl Property for Vibration {}

#[derive(Default)]
pub struct Humidity {
    value: u16,
}

impl Property for Humidity {}

#[derive(Default)]
pub struct Smoke {
    value: u16,
}

impl Property for Smoke {}

#[derive(Default)]
pub struct Sound {
    value: u16,
}
impl Property for Sound {}
