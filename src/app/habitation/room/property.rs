trait Property {
    fn get_value(&self) -> i16;
    fn get_descrition(&self) -> &'static str;
}

struct Temperature {
    value: u16,
}

struct Light {}

struct Radiation {}

struct Vibration {}

struct Humidity {}

struct Smoke {}

struct Sound {}
