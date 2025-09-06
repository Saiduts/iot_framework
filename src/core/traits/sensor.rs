/// Representa un sensor en el sistema (temperatura, humedad, presión, etc.).
///
/// Un sensor puede ser leído para obtener un valor físico o lógico.
///
/// # Associated Types
/// - `Output`: Tipo de datos que produce el sensor.
pub trait Sensor {
    /// Tipo de dato devuelto al leer el sensor.
    type Output;

    /// Lee el valor del sensor.
    ///
    /// # Errores
    /// - `ReadError` si ocurre un fallo en la lectura.
    fn read(&mut self) -> Result<Self::Output, SensorError>;
}

/// Posibles errores de lectura de un sensor.
#[derive(Debug)]
pub enum SensorError {
    /// Fallo en la lectura.
    ReadError(String),
    ParseError(String),
}
