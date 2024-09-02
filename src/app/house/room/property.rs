trait Property {
    fn get_value(&self) -> i16;
    fn get_descrition(&self) -> &str;
}

//os sensores e aparelhos devem comunicar com as propriedades através das suas chaves

struct Temperature {
    value: u16,
}

impl Property for Temperature {}

struct Light {
    value: u16,
}

impl Property for Light {}

struct Radiation {
    value: u16,
}

impl Property for Radiation {}

struct Vibration {
    value: u16,
}

impl Property for Vibration {}

struct Humidity {
    value: u16,
}

impl Property for Humidity {}

struct Smoke {
    value: u16,
}

impl Property for Smoke {}

struct Sound {
    value: u16,
}
impl Property for Sound {}
