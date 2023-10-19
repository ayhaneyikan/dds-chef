
#[derive(Debug, Clone, PartialEq)]
pub enum State {
  CREATED,
  ISSUED,
  EXECUTING,
  COMPLETED,
}

#[cfg(test)]
mod tests {}
