use std::{thread::sleep, time::Duration};

use common::{
    io::{Receiver, Sender},
    msgs::{CommandDone, PrepareCommand, PrepareCommandAck},
    state::State,
    steps::FoodItem,
    topics::TopicName,
};

pub struct PrepChefService {
    prep_item: Option<FoodItem>,
    service_state: State,
    command_receiver: Receiver<PrepareCommand>,
    command_ack_sender: Sender<PrepareCommandAck>,
    command_done_sender: Sender<CommandDone>,
}

impl PrepChefService {
    /// Creates new PrepChefService instance
    pub fn new() -> Self {
        Self {
            prep_item: None,
            service_state: State::CREATED,
            command_receiver: Receiver::new(TopicName::PrepareCommand.to_string(), None),
            command_ack_sender: Sender::new(TopicName::PrepareCommandAck.to_string(), None),
            command_done_sender: Sender::new(TopicName::PrepareCommandDone.to_string(), None),
        }
    }

    /// Returns boolean indicating completion status of the service
    pub fn check_completed(&self) -> bool {
        matches!(self.service_state, State::COMPLETED)
    }
    /// Returns option indicating whether the service failed while "cooking"
    /// ### Returns
    /// - `None` if service is not in `FAILED` state
    /// - `Some(message)` describing the failure if `FAILED`
    pub fn check_failed(&self) -> Option<&str> {
        if let State::FAILED(error_message) = &self.service_state {
            return Some(error_message);
        }
        None
    }

    pub fn cycle(&mut self) {
        match self.service_state {
            State::CREATED => self.on_created(),
            State::ISSUED => self.on_issued(),
            State::EXECUTING => self.on_executing(),
            // shouldn't enter cycle() while COMPLETED
            State::COMPLETED => (),
            // failures checked and printed in main loop
            State::FAILED(_) => (),
        }
    }

    /// After service creation, await command before moving to ISSUED
    fn on_created(&mut self) {
        if let Some(command) = self.command_receiver.receive() {
            self.prep_item = Some(command.get_item());
            self.service_state = State::ISSUED;
            println!("Instructions received, beginning preparations");
            // send ack
            self.command_ack_sender
                .send(PrepareCommandAck)
                .unwrap_or_else(|e| {
                    self.service_state = State::FAILED(format!("Failed to send ack: {}", e));
                });
        }
    }

    /// After service receives command, perform initialization prior to cooking
    fn on_issued(&mut self) {
        match self.prep_item.unwrap() {
            FoodItem::CHICKEN => {
                println!("Gathering spices and setting up cutting board");
                sleep(Duration::from_secs(2));
            }
            FoodItem::SALMON => {
                println!("Preparing baking sheet and getting paprika");
                sleep(Duration::from_secs(3));
            }
        }
        println!(
            "Prep materials gathered for the {}",
            self.prep_item.unwrap()
        );
        self.service_state = State::EXECUTING;
    }

    /// After initialization, perform necessary preparation task
    fn on_executing(&mut self) {
        match self.prep_item.unwrap() {
            FoodItem::CHICKEN => {
                println!("Removing extra fat and seasoning the chicken");
                sleep(Duration::from_secs(5));
            }
            FoodItem::SALMON => {
                println!("Prep salmon on baking sheet and season");
                sleep(Duration::from_secs(3));
            }
        }

        println!("Preparations completed");
        self.service_state = State::COMPLETED;
        // send done indicator
        self.command_done_sender
            .send(CommandDone)
            .unwrap_or_else(|e| {
                self.service_state = State::FAILED(format!("Failed to send done: {}", e));
            });
    }
}


/// Test module for the prep chef
#[cfg(test)]
mod prep_chef_tests {
    use common::state::State;

    use crate::prep_chef::PrepChefService;

    /// Tests initial values of chef state variables 
    #[test]
    fn initialization() {
        let chef = PrepChefService::new();
        assert_eq!(chef.service_state, State::CREATED);
        assert!(!chef.check_completed());
        assert!(chef.check_failed().is_none());
        assert!(chef.prep_item.is_none());
    }

    /// Ensures chef state doesn't progress without receiving a cycle
    /// TODO: revisit. This testing format may not even make sense
    ///         look into best methods for testing infinite loops
    #[test]
    fn state_check_no_command() {
        let mut chef = PrepChefService::new();
        // cycle a large number of times and check state
        for _ in 0..10000 {
            chef.cycle();
            assert_eq!(chef.service_state, State::CREATED);
        }
    }

    /// Checks state progress upon receiving prepare command
    #[test]
    fn state_check_command() {
    }
}
