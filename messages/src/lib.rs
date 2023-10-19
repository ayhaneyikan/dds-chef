use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleMessage {
  val: i32,
  version: f32,
}

impl SimpleMessage {
  pub fn new(val: i32, version: f32) -> Self {
    Self {
      val,
      version
    }
  }
}

#[cfg(test)]
mod tests {
}
