use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleCommand {
    pub value: i32,
    pub version: f32,
}

impl SimpleCommand {
    pub fn new(value: i32, version: f32) -> Self {
        Self { value, version }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleCommandAck;

impl SimpleCommandAck {
    pub fn new() -> Self {
        Self
    }
}
