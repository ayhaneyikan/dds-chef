use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::steps::FoodItem;

/// Message indicating completion of a command
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandDone;

/// Command initiating preparation actions
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

/// CommandAck indicating reception of a prepare command
#[derive(Debug, Serialize, Deserialize)]
pub struct PrepareCommandAck;



/// Command initiating cooking actions
#[derive(Debug, Serialize, Deserialize)]
pub struct CookCommand {
    item: FoodItem,
    time: Duration,
}
impl CookCommand {
    pub fn new(item: FoodItem, time: Duration) -> Self {
        Self { item, time }
    }
    pub fn get_item(&self) -> FoodItem {
        self.item
    }
    pub fn get_duration(&self) -> Duration {
        self.time
    }
}

/// CommandAck indicating reception of a cooking command
#[derive(Debug, Serialize, Deserialize)]
pub struct CookCommandAck {
    command: CookCommand,
}
