use crate::core::traits::sensor::{Sensor, SensorError};
use std::fs;

pub struct Temperature {
    device_path: String,
}

impl Temperature {
    pub fn new(device_id: &str) -> Result<Self, SensorError> {
        Ok(Self {
            device_path: format!("/sys/bus/w1/devices/{}/w1_slave", device_id),
        })
    }

    fn read_temp_raw(&self) -> Result<String, SensorError> {
        fs::read_to_string(&self.device_path)
            .map_err(|e| SensorError::ReadError(format!("Error leyendo archivo: {}", e)))
    }
}

impl Sensor for Temperature {
    type Output = String;

    fn read(&mut self) -> Result<Self::Output, SensorError> {
        let data = self.read_temp_raw()?;
        if let Some(eq_pos) = data.find("t=") {
            let temp_str = &data[eq_pos + 2..].trim();
            let temp_c = temp_str
                .parse::<f32>()
                .map_err(|e| SensorError::ReadError(format!("parse: {}", e)))?
                / 1000.0;
            Ok(format!("{:.2} °C", temp_c)) 
        } else {
            Err(SensorError::ReadError("Formato inesperado en w1_slave".to_string()))
        }
    }
}

