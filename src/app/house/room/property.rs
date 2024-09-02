pub trait Property {
    fn get_value(&self) -> i16;
}

//os sensores e aparelhos devem comunicar com as propriedades através das suas chaves

#[derive(Default)]
pub struct Temperature {
    value: i16,
}

impl Property for Temperature {
    fn get_value(&self) -> i16 {
        self.value
    }
}

#[derive(Default)]
pub struct Light {
    value: i16,
}

impl Property for Light {
    fn get_value(&self) -> i16 {
        self.value
    }
}

#[derive(Default)]
pub struct Radiation {
    value: i16,
}

impl Property for Radiation {
    fn get_value(&self) -> i16 {
        self.value
    }
}

#[derive(Default)]
pub struct Vibration {
    value: i16,
}

impl Property for Vibration {
    fn get_value(&self) -> i16 {
        self.value
    }
}

#[derive(Default)]
pub struct Humidity {
    value: i16,
}

impl Property for Humidity {
    fn get_value(&self) -> i16 {
        self.value
    }
}

#[derive(Default)]
pub struct Smoke {
    value: i16,
}

impl Property for Smoke {
    fn get_value(&self) -> i16 {
        self.value
    }
}

#[derive(Default)]
pub struct Sound {
    value: i16,
}
impl Property for Sound {
    fn get_value(&self) -> i16 {
        self.value
    }
}
