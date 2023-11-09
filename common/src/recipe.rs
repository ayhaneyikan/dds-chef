use serde::{Deserialize, Serialize};

use crate::steps::Step;

/// Represents a cooking recipe. The key portion being the `objectives` vector
/// which stores a list of cooking steps to be carried out by various appliances
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    title: String,
    description: String,
    steps: Vec<Step>,
}

impl Recipe {
    /// Attempts to create a recipe from the given path to
    pub fn from_file(in_file_path: &str) -> Result<Self, String> {
        // ensure given file is a YAML
        if !Self::is_yaml(in_file_path) {
            return Err("Provided recipe file must be a YAML file".to_string());
        }
        // check that file exists
        if !std::path::Path::new(in_file_path).exists() {
            return Err("Unable to locate file at: ".to_string() + in_file_path);
        }
        // read file contents
        match std::fs::read_to_string(in_file_path) {
            Ok(contents) => serde_yaml::from_str::<Self>(&contents).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Attempts to write this recipe to file at the given location
    pub fn to_file(&self, out_file_path: &str) -> Result<(), String> {
        // ensure given file is a YAML
        if !Self::is_yaml(out_file_path) {
            return Err("Provided recipe file must be a YAML file".to_string());
        }
        // ensure no file already exists
        if std::path::Path::new(out_file_path).exists() {
            return Err("Another file already exists at: ".to_string() + out_file_path);
        }
        // serialize and write to file
        match serde_yaml::to_string(&self) {
            Ok(yaml_str) => std::fs::write(out_file_path, yaml_str).map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Checks end of given filename for .yaml or .yml
    fn is_yaml(file_path: &str) -> bool {
        file_path.ends_with(".yaml") || file_path.ends_with(".yml")
    }
}
