use crate::core::traits::actuator::{Actuator, ActuatorError};
use crate::core::SensorOutput;
use crate::Sensor;
/// Actuador dummy que no hace nada
pub struct DummyActuator;

impl DummyActuator {
    pub fn new() -> Self {
        Self
    }
}

impl Actuator for DummyActuator {
    type Command = SensorOutput;

    fn execute(&mut self, _command: Self::Command) -> Result<(), ActuatorError> {
        // No hacer nada - solo un placeholder
        println!("[DUMMY ACTUATOR] Datos recibidos pero no se ejecuta ninguna acción");
        Ok(())
    }
}