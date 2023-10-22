use common::State;
use messages::{SimpleCommand, SimpleCommandAck};
use rustdds::no_key::{DataReader, DataWriter};

pub struct PublisherBase {
    current_state: State,
    command_sender: DataWriter<SimpleCommand>,
    command_ack_receiver: DataReader<SimpleCommandAck>,
}

impl PublisherBase {
    pub fn new(
        command_sender: DataWriter<SimpleCommand>,
        command_ack_receiver: DataReader<SimpleCommandAck>,
    ) -> Self {
        Self {
            current_state: State::CREATED,
            command_sender,
            command_ack_receiver,
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
        self.command_sender.write(data, None).unwrap();

        println!("Command sent...");
    }

    /// attempts to receive command acknowledgement
    ///
    /// marks state as `COMPLETED` if received, otherwise no side effects
    fn attempt_receive_ack(&mut self) {
        // attempt to read in sample
        match self.command_ack_receiver.take_next_sample() {
            // acknowledgement received, service completed
            Ok(Some(_ack)) => self.current_state = State::COMPLETED,
            Ok(None) => (),
            Err(_) => println!("Error receiving ack sample"),
        }
    }
}
