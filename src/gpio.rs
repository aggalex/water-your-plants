use rppal::gpio::{Gpio, OutputPin};

#[derive(Copy, Clone)]
pub enum Led {
    Red = 17,
    Green = 18,
}

impl Led {
    pub fn output(&self) -> rppal::gpio::Result<OutputPin> {
        let output = Gpio::new()?.get(*self as u8)?.into_output();
        Ok(output)
    }
}
