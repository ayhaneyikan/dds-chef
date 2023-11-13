use serde::{Deserialize, Serialize};

use crate::steps::FoodItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct BeginPreparationCommand {
    main_items: Vec<FoodItem>,
    side_items: Vec<FoodItem>,
}























// TODO: DELETE MSGS BELOW

#[derive(Debug, Clone, Serialize, Deserialize)]
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
impl Default for SimpleCommandAck {
    fn default() -> Self {
        Self::new()
    }
}
