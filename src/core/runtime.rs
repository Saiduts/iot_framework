use crate::core::traits::actuator::Actuator;
use crate::core::traits::communicator::Communicator;
use crate::core::traits::sensor::Sensor;
use crate::core::SensorOutput;
use crate::devices::sensors;
use tokio::time::{sleep, Duration};


/// # RuntimeController
/// 
/// Este componente es el **orquestador principal** del framework IoT.
/// Su función es coordinar el flujo de datos entre los sensores, 
/// los actuadores y el mecanismo de comunicación (por ejemplo, MQTT, AMQP, etc.).
/// 
/// Características principales:
/// - Lee datos de los sensores.
/// - Envía los datos a través del comunicador.
/// - Puede accionar dispositivos (actuadores) en base a la información recibida.
/// - Ejecuta este ciclo de manera periódica gracias a un intervalo definido.

pub struct RuntimeController {
    /// Lista de sensores registrados en el runtime.
    /// Cada sensor debe implementar el trait `Sensor` y producir un `SensorOutput`.
    sensors: Vec<Box<dyn Sensor<Output = SensorOutput> + Send>>,
   
    /// Lista opcional de actuadores.
    /// Los actuadores reciben comandos (del mismo tipo que producen los sensores) 
    /// y ejecutan acciones.
    actuators: Option<Vec<Box<dyn Actuator<Command = SensorOutput> + Send>>>,
   
    /// Módulo de comunicación.
    /// Se encarga de transmitir los datos de los sensores hacia el exterior
    /// (por ejemplo, publicarlos en un broker MQTT).
    communicator: Box<dyn Communicator<Command = SensorOutput, Response = ()> + Send>,

    /// Intervalo de tiempo entre cada iteración del ciclo de ejecución.
    interval: Duration,
}

impl RuntimeController {
     /// Crea una nueva instancia de `RuntimeController`.
    ///
    /// # Parámetros
    /// - `sensors`: lista de sensores a gestionar.
    /// - `actuators`: lista opcional de actuadores (puede ser `None` si no hay).
    /// - `communicator`: componente de comunicación a usar.
    /// - `interval`: tiempo en segundos entre cada ejecución del ciclo.
    ///
    /// # Retorna
    /// - Una nueva instancia del controlador de runtime lista para ejecutarse.
    pub fn new(
        sensors: Vec<Box<dyn Sensor<Output = SensorOutput> + Send>>,
        actuators: Option<Vec<Box<dyn Actuator<Command = SensorOutput> + Send>>>,
        communicator: Box<dyn Communicator<Command = SensorOutput, Response = ()> + Send>,
        interval: u64,
    ) -> Self {
        Self {
            sensors,
            actuators,
            communicator,
            interval: Duration::from_secs(interval),
        }
    }

     /// Inicia el ciclo principal del controlador.
    /// 
    /// Este método es **asíncrono** y corre en un bucle infinito:
    /// 1. Lee datos de cada sensor.
    /// 2. Intenta enviar esos datos a través del comunicador.
    /// 3. Si existen actuadores, les pasa los datos para que actúen.
    /// 4. Espera el intervalo configurado antes de repetir el ciclo.
    ///
    /// El ciclo nunca termina (a menos que el proceso se detenga).
    
    pub async fn run(&mut self) {
        loop {
            // Lee datos de cada sensor
            for s in self.sensors.iter_mut() {
                match s.read() {
                    // Si no hay errores, envía los datos al comunicador
                    Ok(output) => {
                        if let Err(e) = self.communicator.send(output.clone()) {
                            eprintln!("Error enviando dato: {:?}", e);
                        }
                        // Si hay actuadores, ejecútanlos
                        if let Some(acts) = &mut self.actuators {
                            for a in acts.iter_mut() {
                                if let Err(e) = a.execute(output.clone()) {
                                    eprintln!("Error actuando: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Error leyendo sensores: {:?}", e),
                }
            }
            sleep(self.interval).await;
        }
    }
}
