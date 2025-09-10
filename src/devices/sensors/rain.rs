use crate::core::traits::sensor::{Sensor, SensorError};
use crate::drivers::gpio::GpioDriver;
use crate::core::SensorOutput;

/// RainSensor: interpreta la salida digital (DO) del módulo de lluvia.
/// Atención: muchos módulos DO = LOW cuando está mojado (active low).
pub struct RainSensor {
    gpio: GpioDriver,
    /// Si el módulo está activo en LOW (true) o en HIGH (false).
    /// Muchos módulos usan active_low = true por defecto.
    active_low: bool,
}

impl RainSensor {
    /// Crea un RainSensor en el pin BCM indicado.
    /// active_low = true si DO = LOW cuando hay agua (común).
    pub fn new(pin: u8, active_low: bool) -> Result<Self, SensorError> {
        let gpio = GpioDriver::new(pin).map_err(|e| SensorError::ReadError(format!("gpio init: {}", e)))?;
        Ok(Self { gpio, active_low })
    }
}

impl Sensor for RainSensor {
    type Output = SensorOutput; // true = MOJADO, false = SECO

    fn read(&mut self) -> Result<Self::Output, SensorError> {
        // read_bool devuelve true si el pin está en HIGH
        let raw_high = self.gpio.read_bool();
        // Si el sensor es active_low, entonces LOW = mojado
        let wet = if self.active_low { !raw_high } else { raw_high };
        Ok(SensorOutput::Text(
            if wet { "HÚMEDO".to_string() } else { "SECO".to_string() }
        ))
    }
}
