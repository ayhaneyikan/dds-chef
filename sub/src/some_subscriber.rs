use messages::{SimpleCommand, SimpleCommandAck};
use rustdds::no_key::{DataWriter, DataReader};

pub struct SubscriberBase {
  completion_status: bool,
  last_command: Option<SimpleCommand>,
  command_receiver: DataReader<SimpleCommand>,
  command_ack_sender: DataWriter<SimpleCommandAck>,
}

impl SubscriberBase {
  pub fn new(command_receiver: DataReader<SimpleCommand>, command_ack_sender: DataWriter<SimpleCommandAck>) -> Self {
    Self {
      completion_status: false,
      last_command: None,
      command_receiver,
      command_ack_sender,
    }
  }

  pub fn check_completed(&self) -> bool { self.completion_status }

  pub fn cycle(&mut self) {
    // attempt to read in a sample
    if let Ok(Some(sample)) = self.command_receiver.take_next_sample() {
      println!("Sample received: {:?}", sample.value());

      // store command received
      self.last_command = Some(sample.into_value());

      // send acknowledgement
      self.command_ack_sender.write(SimpleCommandAck::new(), None).unwrap();

      // process received command
      println!("Some data processing involving the received values: {} {}",
          self.last_command.as_ref().unwrap().value,
          self.last_command.as_ref().unwrap().version);

      self.completion_status = true;
    }
    
    // otherwise no message received
  }
}
