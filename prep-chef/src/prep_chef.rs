use common::{io::Receiver, msgs::BeginPreparationCommand, state::State};

pub struct PrepChefService {
    current_state: State,
    command_receiver: Receiver<BeginPreparationCommand>,
}

impl PrepChefService {
    pub fn new() -> Self {
        Self {
            current_state: State::CREATED,
            command_receiver: Receiver::new("begin_preparation_command".to_string(), None),
        }
    }

    pub fn check_completed(&self) -> bool {
        matches!(self.current_state, State::COMPLETED)
    }

    pub fn cycle(&mut self) {
        todo!()
    }
}
