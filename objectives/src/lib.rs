use serde::{Serialize, Deserialize};

pub mod temp_objective;

#[derive(Debug, Serialize, Deserialize)]
pub enum Objective {
    TEMP,
}