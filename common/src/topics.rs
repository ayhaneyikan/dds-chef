use core::fmt;

pub enum TopicName {
    PrepareCommand,
    PrepareCommandAck,
    PrepareCommandDone,
    CookCommand,
    CookCommandAck,
    CookCommandDone,
}

impl fmt::Display for TopicName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let topic = match self {
            TopicName::PrepareCommand => "prepare_command",
            TopicName::PrepareCommandAck => "prepare_command_ack",
            TopicName::PrepareCommandDone => "prepare_command_done",
            TopicName::CookCommand => "cook_command",
            TopicName::CookCommandAck => "cook_command_ack",
            TopicName::CookCommandDone => "cook_command_done",
        };
        write!(f, "{topic}")
    }
}
