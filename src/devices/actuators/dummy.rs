use crate::core::traits::actuator::{Actuator, ActuatorError};
use crate::devices::sensors::simulated_sensor::SensorData;
/// Actuador dummy que no hace nada
pub struct DummyActuator;

impl DummyActuator {
    pub fn new() -> Self {
        Self
    }
}

impl Actuator for DummyActuator {
    type Command = String;

    fn execute(&mut self, _command: Self::Command) -> Result<(), ActuatorError> {
        // No hacer nada - solo un placeholder
        println!("[DUMMY ACTUATOR] Datos recibidos pero no se ejecuta ninguna acción");
        Ok(())
    }
}