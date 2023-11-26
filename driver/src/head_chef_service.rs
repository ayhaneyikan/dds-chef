use std::{thread::sleep, time::Duration};

use common::{
    io::{Receiver, Sender},
    msgs::{CommandDone, CookCommand, CookCommandAck, PrepareCommand, PrepareCommandAck},
    recipe::Recipe,
    state::State,
    steps::Step,
    topics::TopicName,
};

/// Used within the HeadChefService to track progress executing a recipe
enum RecipeState {
    /// Entrypoint to the executing state, no action currently being taken
    Initial,
    /// Sending a prep command out
    PrepCmd,
    /// Awaiting an ack from the prep command receiver
    PrepAck,
    /// Checking for message indicating completion of the prep step
    PrepDone,
    /// Sending a cook command out
    CookCmd,
    /// Awaiting an ack from the cook command receiver
    CookAck,
    /// Checking for message indicating completion of the cook step
    CookDone,
}

/// Head Chef which oversees "cooking" of the given recipe
///
/// This is done by delegating various steps to relevant appliances
pub struct HeadChefService {
    // recipe management
    recipe: Recipe,
    step_index: usize,
    // service state management
    service_state: State,
    recipe_state: RecipeState,
    // senders / receivers
    prep_command_sender: Sender<PrepareCommand>,
    prep_command_ack_receiver: Receiver<PrepareCommandAck>,
    prep_command_done_receiver: Receiver<CommandDone>,
    cook_command_sender: Sender<CookCommand>,
    cook_command_ack_receiver: Receiver<CookCommandAck>,
    cook_command_done_receiver: Receiver<CommandDone>,
}

impl HeadChefService {
    /// Creates a new instance of the execution control service
    pub fn new(recipe: Recipe) -> Self {
        Self {
            recipe,
            step_index: 0,
            service_state: State::CREATED,
            recipe_state: RecipeState::Initial,
            // senders / receivers instantiation
            prep_command_sender: Sender::new(TopicName::PrepareCommand.to_string(), None),
            prep_command_ack_receiver: Receiver::new(
                TopicName::PrepareCommandAck.to_string(),
                None,
            ),
            prep_command_done_receiver: Receiver::new(
                TopicName::PrepareCommandDone.to_string(),
                None,
            ),
            cook_command_sender: Sender::new(TopicName::CookCommand.to_string(), None),
            cook_command_ack_receiver: Receiver::new(TopicName::CookCommandAck.to_string(), None),
            cook_command_done_receiver: Receiver::new(TopicName::CookCommandDone.to_string(), None),
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

    /// Work to be carried out each iteration of the service
    pub fn cycle(&mut self) {
        match &self.service_state {
            // head chef is automatically issued upon creation
            State::CREATED => self.service_state = State::ISSUED,
            State::ISSUED => self.on_issued(),
            State::EXECUTING => self.on_executing(),
            // shouldn't enter cycle() while COMPLETED
            State::COMPLETED => (),
            // perhaps some future handle failure method?
            State::FAILED(_e) => (),
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
        self.service_state = State::EXECUTING;
    }

    /// After initialization, delegate necessary preparation and cooking tasks
    fn on_executing(&mut self) {
        // check if done with recipe steps yet or not
        let curr_step = match self.recipe.get_steps().get(self.step_index) {
            // recipe completed case
            None => {
                self.service_state = State::COMPLETED;
                return;
            }
            Some(step) => step,
        };

        match self.recipe_state {
            RecipeState::Initial => {
                // transition to appropriate state based on first step
                self.recipe_state = match curr_step {
                    Step::Prepare(_) => RecipeState::PrepCmd,
                    Step::Cook(_, _) => RecipeState::CookCmd,
                };
            }
            RecipeState::PrepCmd => {
                // extract item from step and create command
                let prep_command = match curr_step {
                    Step::Prepare(item) => PrepareCommand::new(*item),
                    _ => {
                        println!("Executing state didn't match current step type");
                        self.recipe_state = RecipeState::Initial;
                        return;
                    }
                };

                println!("Assessing requried preparation tasks");
                self.recipe_state = RecipeState::PrepAck;
                // send out command
                self.prep_command_sender
                    .send(prep_command)
                    .unwrap_or_else(|e| {
                        self.service_state =
                            State::FAILED(format!("Failed to send prep command: {}", e));
                    });
            }
            RecipeState::PrepAck => {
                // check to receive ack
                if let Some(_ack) = self.prep_command_ack_receiver.receive() {
                    self.recipe_state = RecipeState::PrepDone;
                    println!("Preparation tasks assigned to another chef");
                }
            }
            RecipeState::PrepDone => {
                // check for completed message
                if let Some(_ack) = self.prep_command_done_receiver.receive() {
                    self.step_index += 1;
                    self.recipe_state = RecipeState::Initial;
                    println!("Preparations complete!");
                }
            }
            RecipeState::CookCmd => {
                // extract item and duration from step and create command
                let cook_command = match curr_step {
                    Step::Cook(item, time) => CookCommand::new(*item, *time),
                    _ => {
                        println!("Executing state didn't match current step type");
                        self.recipe_state = RecipeState::Initial;
                        return;
                    }
                };

                self.recipe_state = RecipeState::CookAck;
                println!("Assessing requried cooking tasks");
                // send out command
                self.cook_command_sender
                    .send(cook_command)
                    .unwrap_or_else(|e| {
                        self.service_state =
                            State::FAILED(format!("Failed to send cook command: {}", e));
                    });
            }
            RecipeState::CookAck => {
                // check to receive ack
                if let Some(_ack) = self.cook_command_ack_receiver.receive() {
                    self.recipe_state = RecipeState::CookDone;
                    println!("Cooking tasks assigned to another chef");
                }
            }
            RecipeState::CookDone => {
                // check for completed message
                if let Some(_ack) = self.cook_command_done_receiver.receive() {
                    self.step_index += 1;
                    self.recipe_state = RecipeState::Initial;
                    println!("Cooking complete!");
                }
            }
        };
    }
}
