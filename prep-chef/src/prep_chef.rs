use std::{thread::sleep, time::Duration};

use common::{io::Receiver, msgs::PrepareCommand, state::State, steps::FoodItem};

pub struct PrepChefService {
    current_state: State,
    command_receiver: Receiver<PrepareCommand>,
    prep_item: Option<FoodItem>,
}

impl PrepChefService {
    /// Creates new PrepChefService instance
    pub fn new() -> Self {
        Self {
            current_state: State::CREATED,
            command_receiver: Receiver::new("begin_preparation_command".to_string(), None),
            prep_item: None,
        }
    }

    pub fn check_completed(&self) -> bool {
        matches!(self.current_state, State::COMPLETED)
    }

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
            self.prep_item = Some(command.get_item());
            self.current_state = State::ISSUED;
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
        self.current_state = State::EXECUTING;
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
        self.current_state = State::COMPLETED;
    }
}
