use common::{
    io::{Receiver, Sender},
    msgs::{SimpleCommand, SimpleCommandAck},
};

pub struct SubscriberBase {
    completion_status: bool,
    last_command: Option<SimpleCommand>,
    command_receiver: Receiver<SimpleCommand>,
    command_ack_sender: Sender<SimpleCommandAck>,
}

impl SubscriberBase {
    pub fn new() -> Self {
        Self {
            completion_status: false,
            last_command: None,
            command_receiver: Receiver::new("simple_command".to_string(), None),
            command_ack_sender: Sender::new("simple_command_ack".to_string(), None),
        }
    }

    pub fn check_completed(&self) -> bool {
        self.completion_status
    }

    pub fn cycle(&mut self) {
        // attempt to read in a sample
        if let Some(sample) = self.command_receiver.receive() {
            println!("Sample received: {:?}", &sample);

            // store command received
            self.last_command = Some(sample);

            // send acknowledgement
            self.command_ack_sender.send(SimpleCommandAck::new());

            // process received command
            println!(
                "Some data processing involving the received values: {} {}",
                self.last_command.as_ref().unwrap().value,
                self.last_command.as_ref().unwrap().version
            );

            self.completion_status = true;
        }
        // TODO: handle error case?

        // otherwise no message received
    }
}
