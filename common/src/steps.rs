use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};

/// Represents possible steps in a cooking recipe
#[derive(Debug, Serialize, Deserialize)]
pub enum Step {
    Prepare(FoodItem),
    /// Cook given food item for given time
    Cook(FoodItem, Duration),
}

/// Describes the space of possible food items that may make up a recipe
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FoodItem {
    CHICKEN,
    SALMON,
}

impl Display for FoodItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FoodItem::CHICKEN => write!(f, "Chicken"),
            FoodItem::SALMON => write!(f, "Salmon"),
        }
    }
}
