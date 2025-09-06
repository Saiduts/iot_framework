use crate::core::traits::communicator::{Communicator, CommunicatorError};
use crate::devices::sensors::simulated_sensor::SensorData;

/// `ConsoleCommunicator` es un comunicador simple que envía datos a la salida estándar (consola).
///
/// Este componente implementa el trait [`Communicator`] y se utiliza principalmente
/// para depuración o ejecución local, permitiendo visualizar los datos que serían enviados
/// a un sistema de comunicación real.
///
pub struct ConsoleCommunicator;

impl Communicator for ConsoleCommunicator {
    /// El tipo de datos que se enviará al comunicador.
    type Command = String;
    /// El tipo de datos que se recibirá como respuesta.
    type Response = ();

    /// Envía un comando a la consola imprimiéndolo con un prefijo identificador.
    ///
    /// # Parámetros
    /// - `command`: Cadena de texto a enviar/imprimir.
    ///
    /// # Retorna
    /// - `Ok(())` si el mensaje fue impreso correctamente.
    /// - [`CommunicatorError`] en caso de error (no se esperan errores en implementación local).
    ///
    /// # Ejemplo
    /// ```
    /// let mut console_comm = ConsoleCommunicator;
    /// console_comm.send("Temperatura: 25°C".to_string()).unwrap();
    /// ```
    /// Envía datos del sensor a la consola con formato legible.
    fn send(&mut self, command: Self::Command) -> Result<Self::Response, CommunicatorError> {
        println!("[CONSOLE] 🌡️  Temp: {:.2}°C | 💧 Humedad: {:.2}% | ⏰ Timestamp: {}", 
                 command.temperature, command.humidity, command.timestamp);
        Ok(())
    }

    fn receive(&mut self) -> Result<Self::Response, CommunicatorError> {
        unimplemented!()
    }
}

impl ConsoleCommunicator {
    pub fn new() -> Self {
        Self
    }
}