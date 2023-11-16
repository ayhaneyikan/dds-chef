use std::{thread::sleep, time::Duration};

use common::{
    io::{Receiver, Sender},
    msgs::{PrepareCommand, PrepareCommandAck, SimpleCommand, SimpleCommandAck},
    recipe::Recipe,
    state::State,
    steps::Step,
};

/// Head Chef which oversees "cooking" of the given recipe
///
/// This is done by delegating various steps to relevant appliances
pub struct HeadChefService {
    recipe: Recipe,
    step_index: usize,
    awaiting_ack: bool,
    current_state: State,
    // senders / receivers
    prep_command_sender: Sender<PrepareCommand>,
    prep_command_ack_receiver: Receiver<PrepareCommandAck>,
}

impl HeadChefService {
    /// Creates a new instance of the execution control service
    pub fn new(recipe: Recipe) -> Self {
        Self {
            recipe,
            step_index: 0,
            awaiting_ack: false,
            current_state: State::CREATED,
            prep_command_sender: Sender::new("prepare_command".to_string(), None),
            prep_command_ack_receiver: Receiver::new("prepare_command_ack".to_string(), None),
        }
    }

    /// Returns boolean indicating whether the recipe is still being "cooked"
    pub fn check_completed(&self) -> bool {
        matches!(self.current_state, State::COMPLETED)
    }

    /// Work to be carried out each iteration of the service
    pub fn cycle(&mut self) {
        match self.current_state {
            // head chef is automatically issued upon creation
            State::CREATED => self.current_state = State::ISSUED,
            State::ISSUED => self.on_issued(),
            State::EXECUTING => self.on_executing(),
            // shouldn't enter cycle() while COMPLETED
            State::COMPLETED => (),
        }
    }

    /// After service creation, begin initialization prior to preparing recipe
    fn on_issued(&mut self) {
        println!("Head chef opening up the restaurant for the evening");
        sleep(Duration::from_secs(15));
        println!(
            "The recipe for tonight appears to be {}",
            self.recipe.get_title()
        );
        self.current_state = State::EXECUTING;
    }

    /// After initialization, delegate necessary preparation and cooking tasks
    fn on_executing(&mut self) {
        let curr_step;

        // check if done with recipe steps yet or not
        match self.recipe.get_steps().get(self.step_index) {
            // completed case
            None => {
                self.current_state = State::COMPLETED;
                return;
            }
            Some(step) => curr_step = step,
        }

        // TODO: LOGIC REGARDING AWAITING ACK AND SENDING MESSAGES BELOW

        match curr_step {
            Step::Prepare(item) => {
                let prep_command = PrepareCommand::new(item.clone());
            }
            Step::Cook(_, _) => todo!(),
        }
    }

    /// Creates and sends command message
    ///
    /// Marks state as `EXECUTING`
    fn send_msg(&mut self) {
        self.current_state = State::EXECUTING;

        // create and send command
        let data = SimpleCommand::new(50, 4.4);
        // TODO: implement thorough error handling
        // self.prep_command_sender.send(data);

        println!("Command sent...");
    }

    /// Attempts to receive command acknowledgement
    ///
    /// Marks state as `COMPLETED` if received, otherwise no side effects
    fn attempt_receive_ack(&mut self) {
        // attempt to read in sample
        if let Some(_ack) = self.prep_command_ack_receiver.receive() {
            // acknowledgement received, service completed
            self.current_state = State::COMPLETED;
        }
    }
}
