use objectives::Objective;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MissionPlan {
    title: String,
    description: String,
    objectives: Vec<Objective>,
}
