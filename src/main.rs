// use crate::config::loader::load_config;

// fn main() {
//     match load_config("config.toml") {
//         Ok(config) => {
//             println!("Device name: {}", config.device.name);
//             println!("Device location: {}", config.device.location);
//             println!("Sensor type: {}", config.sensor.r#type_);
//             println!("Sensor pin: {}", config.sensor.pin);
//             println!("Sensor unit: {}", config.sensor.unit);
//             println!("Actuator type: {}", config.actuator.r#type_);
//             println!("Actuator pin: {}", config.actuator.pin);
//             println!("Communication type: {}", config.communication.r#type_);
//             println!("Communication broker URL: {}", config.communication.broker_url);
//             println!("Communication topic: {}", config.communication.topic);
//             println!("Storage type: {}", config.storage.r#type_);
//             println!("Storage path: {}", config.storage.path);
//             println!("Runtime interval (ms): {}", config.runtime.interval_ms);
//         }
//         Err(e) => {
//             println!("Error loading config: {}", e);
//         }
//     }
// }

// use iot_framework::core::traits::communicator::{self, Communicator};
// use iot_framework::core::traits::sensor;
// use iot_framework::network::console::ConsoleCommunicator;
// use iot_framework::network::mqtt::MqttCommunicator;
// use iot_framework::SimulatedSensor;

// fn main() {
    
//     let sensor = SimulatedSensor::new();
//     let mut communicator = ConsoleCommunicator;

//     let temp = sensor.read_temperature();
//     let hum = sensor.read_humidity();
//     let timestamp = sensor.read_timestamp();

//     let mensaje = format!("Temperatura: {}°C, Humedad: {}%, Fecha: {}", temp, hum, timestamp);

//     if let Err(e) = communicator.send(mensaje) {
//         eprintln!("Error sending message: {:?}", e);
//     }
// }

// src/main.rs










// mod config;
// mod core;
// mod devices;
// mod network;
// mod platform;

// use std::error::Error;
// use core::runtime::RuntimeController;
// use std::vec;
// use devices::sensors::simulated_sensor::SimulatedSensor;
// use network::console::ConsoleCommunicator;
// use devices::actuators::dummy::DummyActuator;

// #[tokio::main] // Necesario para usar Tokio
// async fn main() -> Result<(), Box<dyn Error>> {
//     println!("Iniciando Framework IoT...");
    
//     let sensor =  SimulatedSensor::new();

//     let actuator = DummyActuator::new();

//     let communicator = ConsoleCommunicator::new();

//     let mut runtime = RuntimeController::new(
//         vec![sensor],
//         vec![actuator],
//         communicator,
//         5 // intervalo en segundos
//     );

//     println!("Runtime configurado. Iniciando ciclo de ejecución...");
    

//     runtime.run().await;
//     Ok(())
// }



// src/main.rs
mod drivers;
mod devices;
mod core;
use core::traits::sensor::Sensor;
use devices::sensors::rain::RainSensor;
use std::{thread, time::Duration};

fn main() {
    // Pin BCM que usaremos (ejemplo: GPIO17)
    let pin_bcm: u8 = 17;
    // Muchos módulos DO = LOW cuando mojado, así que active_low=true
    let mut sensor = match RainSensor::new(pin_bcm, true) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No se pudo inicializar RainSensor: {:?}", e);
            return;
        }
    };

    println!("Leyendo sensor de lluvia en GPIO{} (Ctrl+C para salir)...", pin_bcm);
    loop {
        match sensor.read() {
            Ok(true) => println!("ESTADO: MOJADO"),
            Ok(false) => println!("ESTADO: SECO"),
            Err(err) => eprintln!("Error lectura: {:?}", err),
        }
        thread::sleep(Duration::from_secs(1));
    }
}
