
pub enum TopicName {
    PrepareCommand,
    PrepareCommandAck,
    PrepareCommandDone,
    CookCommand,
    CookCommandAck,
    CookCommandDone,
}

impl TopicName {
    pub fn to_string(&self) -> String {
        match self {
            TopicName::PrepareCommand => "prepare_command",
            TopicName::PrepareCommandAck => "prepare_command_ack",
            TopicName::PrepareCommandDone => "prepare_command_done",
            TopicName::CookCommand => "cook_command",
            TopicName::CookCommandAck => "cook_command_ack",
            TopicName::CookCommandDone => "cook_command_done",
        }.to_string()
    }
}