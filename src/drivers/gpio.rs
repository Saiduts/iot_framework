// src/drivers/gpio.rs
use rppal::gpio::{Gpio, InputPin, Level};
use std::error::Error;

/// Driver mínimo y seguro para leer un pin digital en Raspberry Pi.
pub struct GpioDriver {
    pin: InputPin,
    pub pin_number: u8,
}

impl GpioDriver {
    /// Crea un nuevo driver para el pin BCM indicado.
    /// Devuelve Err si rppal falla (pin inválido, permisos, etc.).
    pub fn new(pin_number: u8) -> Result<Self, Box<dyn Error>> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(pin_number)?.into_input_pullup();

        Ok(Self { pin, pin_number})
    }

    /// Lee el nivel físico (High/Low).
    pub fn read_level(&self) -> Level {
        self.pin.read()
    }

    /// Lee y devuelve booleano: true = HIGH, false = LOW
    pub fn read_bool(&self) -> bool {
        self.read_level() == Level::High
    }
}
