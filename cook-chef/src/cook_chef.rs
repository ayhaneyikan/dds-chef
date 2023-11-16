use std::{thread::sleep, time::Duration};

use common::{io::Receiver, msgs::CookCommand, state::State, steps::FoodItem};

pub struct CookChefService {
    current_state: State,
    command_receiver: Receiver<CookCommand>,
    cooking_item: Option<FoodItem>,
    cooking_time: Option<Duration>,
}

impl CookChefService {
    /// Creates new CookChefService instance
    pub fn new() -> Self {
        Self {
            current_state: State::CREATED,
            command_receiver: Receiver::new("begin_cooking_command".to_string(), None),
            cooking_item: None,
            cooking_time: None,
        }
    }

    /// Returns boolean indicating completion status of the service
    pub fn check_completed(&self) -> bool {
        matches!(self.current_state, State::COMPLETED)
    }

    /// Delegates necessary tasks per cycle based on current service state.
    /// Expects helper methods to manage necessary state updates.
    pub fn cycle(&mut self) {
        match self.current_state {
            State::CREATED => self.on_created(),
            State::ISSUED => self.on_issued(),
            State::EXECUTING => self.on_executing(),
            // shouldn't enter cycle() while COMPLETED
            State::COMPLETED => (),
        }
    }

    /// After service creation, await command before moving to ISSUED
    fn on_created(&mut self) {
        if let Some(command) = self.command_receiver.receive() {
            self.cooking_item = Some(command.get_item());
            self.cooking_time = Some(command.get_duration());
            self.current_state = State::ISSUED;
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
        self.current_state = State::EXECUTING;
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
        self.current_state = State::COMPLETED;
    }
}
