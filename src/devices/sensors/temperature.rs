use crate::core::traits::sensor::{Sensor, SensorError};
use std::fs;
use crate::core::SensorOutput;


/// Representa un **sensor de temperatura** que obtiene datos
/// desde el sistema de archivos expuesto por el driver **OneWire** en Linux.
///
/// En sistemas como Raspberry Pi, los sensores **DS18B20** suelen aparecer
/// en la ruta `/sys/bus/w1/devices/{id}/w1_slave`.
///
/// Este módulo abstrae la lectura de ese archivo y lo expone a través del trait `Sensor`.
pub struct Temperature {
     /// Ruta en el sistema de archivos donde se encuentra la información del sensor.
    /// Ejemplo: `/sys/bus/w1/devices/28-00000abcdef/w1_slave`
    device_path: String,
}

impl Temperature {

    /// # Retorna
    /// - `Ok(Temperature)` si la ruta fue construida correctamente.
    /// - `Err(SensorError)` en caso de error (aunque aquí en realidad siempre devuelve `Ok`,
    ///   el `Result` está para mantener consistencia y permitir validaciones futuras).
    pub fn new(device_id: &str) -> Result<Self, SensorError> {
        Ok(Self {
            device_path: format!("/sys/bus/w1/devices/{}/w1_slave", device_id),
        })
    }
    
    /// Lee directamente el archivo `w1_slave` que contiene la salida cruda del sensor.
    ///
    /// # Retorna
    /// - `Ok(String)` con el contenido del archivo.
    /// - `Err(SensorError::ReadError)` si ocurre un problema al leer.
    fn read_temp_raw(&self) -> Result<String, SensorError> {
        fs::read_to_string(&self.device_path)
            .map_err(|e| SensorError::ReadError(format!("Error leyendo archivo: {}", e)))
    }
}

impl Sensor for Temperature {
    type Output = SensorOutput;


    /// Realiza una lectura del sensor.
    ///
    /// Flujo:
    /// 1. Llama a `read_temp_raw` para obtener los datos crudos del archivo.
    /// 2. Busca la cadena `"t="`, que es donde el kernel expone el valor en miligrados Celsius.
    /// 3. Convierte ese valor a `f32` y lo pasa de **miligrados** a **grados Celsius** dividiendo entre 1000.
    /// 4. Devuelve el resultado formateado como `SensorOutput::Text("XX.XX °C")`.
    ///
    /// # Retorna
    /// - `Ok(SensorOutput::Text)` con la temperatura en grados Celsius.
    /// - `Err(SensorError::ReadError)` si el formato no es el esperado o si ocurre un fallo en el parseo.
    fn read(&mut self) -> Result<Self::Output, SensorError> {
        // 1. Leer datos crudos del archivo
        let data = self.read_temp_raw()?;
        // 2. Buscar la posición del texto "t=" en la salida
        if let Some(eq_pos) = data.find("t=") {
            // Extraer el número crudo después de "t="
            let temp_str = &data[eq_pos + 2..].trim();
            // 3. Parsear el valor crudo a `f32` y dividir entre 1000
            let temp_c = temp_str
                .parse::<f32>()
                .map_err(|e| SensorError::ReadError(format!("parse: {}", e)))?
                / 1000.0;
            // 4. Retornar el valor ya convertido y formateado
            Ok(SensorOutput::Text(format!("{:.2} °C", temp_c)))
        } else {
            Err(SensorError::ReadError("Formato inesperado en w1_slave".to_string()))
        }
    }
}

