use std::{thread::sleep, time::Duration};

use common::{
    io::{Receiver, Sender},
    msgs::{CommandDone, CookCommand, CookCommandAck},
    state::State,
    steps::FoodItem,
};

pub struct CookChefService {
    cooking_item: Option<FoodItem>,
    cooking_time: Option<Duration>,
    service_state: State,
    command_receiver: Receiver<CookCommand>,
    command_ack_sender: Sender<CookCommandAck>,
    command_done_sender: Sender<CommandDone>,
}

impl CookChefService {
    /// Creates new CookChefService instance
    pub fn new() -> Self {
        Self {
            cooking_item: None,
            cooking_time: None,
            service_state: State::CREATED,
            command_receiver: Receiver::new("cook_command".to_string(), None),
            command_ack_sender: Sender::new("cook_command_ack".to_string(), None),
            command_done_sender: Sender::new("cook_command_done".to_string(), None),
        }
    }

    /// Returns boolean indicating completion status of the service
    pub fn check_completed(&self) -> bool {
        matches!(self.service_state, State::COMPLETED)
    }
    /// Returns boolean indicating whether the service failed while "cooking"
    pub fn check_failed(&self) -> bool {
        matches!(self.service_state, State::FAILED(_))
    }
    /// Returns string containing explanation of reason for failure
    /// ### Returns
    /// - `None` if service is not in `FAILED` state
    /// - `Some(message)` describing the failure if `FAILED`
    pub fn get_failure_msg(&self) -> Option<&str> {
        if let State::FAILED(error_message) = &self.service_state {
            return Some(error_message);
        }
        None
    }

    /// Delegates necessary tasks per cycle based on current service state.
    /// Expects helper methods to manage necessary state updates.
    pub fn cycle(&mut self) {
        match &self.service_state {
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
            self.cooking_item = Some(command.get_item());
            self.cooking_time = Some(command.get_duration());
            self.service_state = State::ISSUED;
            println!("Instructions received, beginning cooking");
            // send ack
            self.command_ack_sender
                .send(CookCommandAck)
                .unwrap_or_else(|e| {
                    self.service_state = State::FAILED(format!("Failed to send ack: {}", e));
                });
        }
    }

    /// After service receives command, perform initialization prior to cooking
    fn on_issued(&mut self) {
        match self.cooking_item.unwrap() {
            FoodItem::CHICKEN => {
                println!("Heating pan prior to cooking the chicken");
                sleep(Duration::from_secs(5));
            }
            FoodItem::SALMON => {
                println!("Preheating oven prior to cooking the salmon");
                sleep(Duration::from_secs(15));
            }
        }
        println!("Preheating completed");
        self.service_state = State::EXECUTING;
    }

    /// After initialization, perform necessary cooking task
    fn on_executing(&mut self) {
        match self.cooking_item.unwrap() {
            FoodItem::CHICKEN => {
                println!("Cooking chicken on the pan");
                sleep(self.cooking_time.unwrap());
            }
            FoodItem::SALMON => {
                println!("Cooking salmon in the oven");
                sleep(self.cooking_time.unwrap());
            }
        }
        println!("Cooking completed");
        self.service_state = State::COMPLETED;
        // send done indicator
        self.command_done_sender
            .send(CommandDone)
            .unwrap_or_else(|e| {
                self.service_state = State::FAILED(format!("Failed to send done: {}", e));
            });
    }
}
