/// Defines the states a control service transitions through during execution
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    CREATED,
    ISSUED,
    EXECUTING,
    COMPLETED,
}
