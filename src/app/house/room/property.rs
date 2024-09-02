pub trait Property {
    fn get_value(&self) -> i16;
    fn get_descrition(&self) -> &str;
}

//os sensores e aparelhos devem comunicar com as propriedades através das suas chaves

pub struct Temperature {
    value: u16,
}

impl Property for Temperature {}

pub struct Light {
    value: u16,
}

impl Property for Light {}

pub struct Radiation {
    value: u16,
}

impl Property for Radiation {}

pub struct Vibration {
    value: u16,
}

impl Property for Vibration {}

pub struct Humidity {
    value: u16,
}

impl Property for Humidity {}

pub struct Smoke {
    value: u16,
}

impl Property for Smoke {}

pub struct Sound {
    value: u16,
}
impl Property for Sound {}
