use serde::{Deserialize, Serialize};

/// Represents possible steps in a cooking recipe
#[derive(Debug, Serialize, Deserialize)]
pub enum Step {
    /// Preheat oven to provided temperature
    PreheatOven(u8),
    /// Bake given food item for given time
    Bake(FoodItem, u32),
}

/// Describes the space of possible food items that may make up a recipe
#[derive(Debug, Serialize, Deserialize)]
pub enum FoodItem {
    CHICKEN,
    SALMON,
}
