pub trait Property {
    type Output;
    fn get_value(&self) -> Self::Output;
}

//os sensores e aparelhos devem comunicar com as propriedades através das suas chaves

#[derive(Default)]
pub struct Temperature {
    value: u16,
}

impl Property for Temperature {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}

#[derive(Default)]
pub struct Light {
    value: u16,
}

impl Property for Light {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}

#[derive(Default)]
pub struct Radiation {
    value: u16,
}

impl Property for Radiation {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}

#[derive(Default)]
pub struct Vibration {
    value: u16,
}

impl Property for Vibration {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}

#[derive(Default)]
pub struct Humidity {
    value: u16,
}

impl Property for Humidity {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}

#[derive(Default)]
pub struct Smoke {
    value: u16,
}

impl Property for Smoke {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}

#[derive(Default)]
pub struct Sound {
    value: u16,
}
impl Property for Sound {
    type Output = u16;

    fn get_value(&self) -> Self::Output {
        self.value
    }
}
