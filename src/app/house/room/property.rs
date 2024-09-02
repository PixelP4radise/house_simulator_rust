pub trait Property {
    fn get_value(&self) -> u16;
}

//os sensores e aparelhos devem comunicar com as propriedades através das suas chaves

#[derive(Default)]
pub struct Temperature {
    value: u16,
}

impl Property for Temperature {
    fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Default)]
pub struct Light {
    value: u16,
}

impl Property for Light {
    fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Default)]
pub struct Radiation {
    value: u16,
}

impl Property for Radiation {
    fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Default)]
pub struct Vibration {
    value: u16,
}

impl Property for Vibration {
    fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Default)]
pub struct Humidity {
    value: u16,
}

impl Property for Humidity {
    fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Default)]
pub struct Smoke {
    value: u16,
}

impl Property for Smoke {
    fn get_value(&self) -> u16 {
        self.value
    }
}

#[derive(Default)]
pub struct Sound {
    value: u16,
}
impl Property for Sound {
    fn get_value(&self) -> u16 {
        self.value
    }
}
