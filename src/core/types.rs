#[derive(Debug, Clone)]
pub enum SensorOutput {
    Bool(bool),
    Int(i64),
    Float(f32),
    Text(String),
    Bytes(Vec<u8>),      
}
