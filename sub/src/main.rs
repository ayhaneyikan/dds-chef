mod some_subscriber;
use messages::{SimpleCommand, SimpleCommandAck};
use rustdds::{
    CDRDeserializerAdapter, CDRSerializerAdapter, DomainParticipant, QosPolicyBuilder, TopicKind,
};
use some_subscriber::SubscriberBase;

fn main() {
    // set up domain & quality of service policy
    let domain_participant = DomainParticipant::new(0).unwrap();
    let qos = QosPolicyBuilder::new().build();

    // create command topic
    let command_topic = domain_participant
        .create_topic(
            "test_command".to_string(),
            "Simple command test".to_string(),
            &qos,
            TopicKind::NoKey,
        )
        .unwrap();

    // create command ack topic
    let command_ack_topic = domain_participant
        .create_topic(
            "test_command_ack".to_string(),
            "Ack for simple command test".to_string(),
            &qos,
            TopicKind::NoKey,
        )
        .unwrap();

    // create receiver for command topic
    let subscriber = domain_participant.create_subscriber(&qos).unwrap();
    let command_receiver = subscriber
        .create_datareader_no_key::<SimpleCommand, CDRDeserializerAdapter<_>>(&command_topic, None)
        .unwrap();

    // create sender for command ack topic
    let publisher = domain_participant.create_publisher(&qos).unwrap();
    let command_ack_sender = publisher
        .create_datawriter_no_key::<SimpleCommandAck, CDRSerializerAdapter<_>>(
            &command_ack_topic,
            None,
        )
        .unwrap();

    // initialize subscriber service
    let mut s = SubscriberBase::new(command_receiver, command_ack_sender);

    println!("Running subscriber");
    while !s.check_completed() {
        s.cycle();
    }

    println!("Subscriber completed running successfully!");
}
