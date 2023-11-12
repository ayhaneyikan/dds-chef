use serde::{Deserialize, Serialize};

/// Represents possible steps in a cooking recipe
#[derive(Debug, Serialize, Deserialize)]
pub enum Step {
    PreheatOven,
    
}
