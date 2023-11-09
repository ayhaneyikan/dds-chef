use common::{
    io::{Receiver, Sender},
    msgs::{SimpleCommand, SimpleCommandAck},
    recipe::Recipe,
    state::State,
};

const DOMAIN_ID: u16 = 0;

pub struct HeadChefControlService {
    command_sender: Sender<SimpleCommand>,
    command_ack_receiver: Receiver<SimpleCommandAck>,
    current_state: State,
    recipe: Recipe,
}

impl HeadChefControlService {
    /// Creates a new instance of the execution control service
    pub fn new(recipe: Recipe) -> Self {
        Self {
            command_sender: Sender::new(DOMAIN_ID, String::from("simple_command"), None),
            command_ack_receiver: Receiver::new(
                DOMAIN_ID,
                String::from("simple_command_ack"),
                None,
            ),
            current_state: State::CREATED,
            recipe,
        }
    }

    pub fn check_completed(&self) -> bool {
        matches!(self.current_state, State::COMPLETED)
    }

    pub fn cycle(&mut self) {
        match self.current_state {
            State::CREATED => self.send_msg(),
            State::ISSUED => todo!(),
            State::EXECUTING => self.attempt_receive_ack(),

            State::COMPLETED => (), // should never cycle if completed
        }
    }

    /// creates and sends command message
    ///
    /// marks state as `EXECUTING`
    fn send_msg(&mut self) {
        self.current_state = State::EXECUTING;

        // create and send command
        let data = SimpleCommand::new(50, 4.4);
        // TODO: implement thorough error handling
        self.command_sender.send(data);

        println!("Command sent...");
    }

    /// attempts to receive command acknowledgement
    ///
    /// marks state as `COMPLETED` if received, otherwise no side effects
    fn attempt_receive_ack(&mut self) {
        // attempt to read in sample
        if let Some(_ack) = self.command_ack_receiver.receive() {
            // acknowledgement received, service completed
            self.current_state = State::COMPLETED;
        }
    }
}
