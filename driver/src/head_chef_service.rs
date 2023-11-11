use common::{
    io::{Receiver, Sender},
    msgs::{SimpleCommand, SimpleCommandAck},
    recipe::Recipe,
    state::State,
};

/// Head Chef which oversees "cooking" of the given recipe
///
/// This is done by delegating various steps to relevant appliances
pub struct HeadChefService {
    command_sender: Sender<SimpleCommand>,
    command_ack_receiver: Receiver<SimpleCommandAck>,
    current_state: State,
    recipe: Recipe,
}

impl HeadChefService {
    /// Creates a new instance of the execution control service
    pub fn new(recipe: Recipe) -> Self {
        Self {
            command_sender: Sender::new("simple_command".to_string(), None),
            command_ack_receiver: Receiver::new("simple_command_ack".to_string(), None),
            current_state: State::CREATED,
            recipe,
        }
    }

    /// Returns boolean indicating whether the recipe is still being "cooked"
    pub fn check_completed(&self) -> bool {
        matches!(self.current_state, State::COMPLETED)
    }

    /// Work to be carried out each iteration of the service
    pub fn cycle(&mut self) {
        match self.current_state {
            State::CREATED => self.handle_created(),
            State::ISSUED => (),    // will never occur in the Head Chef
            State::EXECUTING => self.attempt_receive_ack(),
            State::COMPLETED => (), // should not cycle if COMPLETED
        }
    }

    /// Carries out necessary service initialization when the service is created
    fn handle_created(&mut self) {
        println!("Chef clocked in, studying a new recipe: {}", self.recipe.get_title());

        println!("Updating menus with a thorough description: {}", self.recipe.get_description());

        // TODO processing the recipe

        self.current_state = State::EXECUTING;
    }


    /// Creates and sends command message
    ///
    /// Marks state as `EXECUTING`
    fn send_msg(&mut self) {
        self.current_state = State::EXECUTING;

        // create and send command
        let data = SimpleCommand::new(50, 4.4);
        // TODO: implement thorough error handling
        self.command_sender.send(data);

        println!("Command sent...");
    }

    /// Attempts to receive command acknowledgement
    ///
    /// Marks state as `COMPLETED` if received, otherwise no side effects
    fn attempt_receive_ack(&mut self) {
        // attempt to read in sample
        if let Some(_ack) = self.command_ack_receiver.receive() {
            // acknowledgement received, service completed
            self.current_state = State::COMPLETED;
        }
    }
}
