use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::steps::FoodItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareCommand {
    item: FoodItem,
}

impl PrepareCommand {
    pub fn new(item: FoodItem) -> Self {
        Self { item }
    }

    pub fn get_item(&self) -> FoodItem {
        self.item
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareCommandAck {
    command: PrepareCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookCommand {
    item: FoodItem,
    time: Duration,
}

impl CookCommand {
    pub fn get_item(&self) -> FoodItem {
        self.item
    }
    pub fn get_duration(&self) -> Duration {
        self.time
    }
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
