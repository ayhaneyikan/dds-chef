mod some_publisher;
use std::{thread::sleep, time::Duration};

use messages::{SimpleCommand, SimpleCommandAck};
use rustdds::{
    CDRDeserializerAdapter, CDRSerializerAdapter, DomainParticipant, QosPolicyBuilder, TopicKind,
};
use some_publisher::PublisherBase;

fn main() {
    // set up domain & quality of service policy
    let participant = DomainParticipant::new(0).unwrap();
    let qos = QosPolicyBuilder::new().build();

    // create command topic
    let command_topic = participant
        .create_topic(
            "test_command".to_string(),
            "Simple command test".to_string(),
            &qos,
            TopicKind::NoKey,
        )
        .unwrap();

    // create command ack topic
    let command_ack_topic = participant
        .create_topic(
            "test_command_ack".to_string(),
            "Ack for simple command test".to_string(),
            &qos,
            TopicKind::NoKey,
        )
        .unwrap();

    // create sender for command topic
    let publisher = participant.create_publisher(&qos).unwrap();
    let command_sender = publisher
        .create_datawriter_no_key::<SimpleCommand, CDRSerializerAdapter<_>>(&command_topic, None)
        .unwrap();

    // create receiver for command ack topic
    let subscriber = participant.create_subscriber(&qos).unwrap();
    let command_ack_receiver = subscriber
        .create_datareader_no_key::<SimpleCommandAck, CDRDeserializerAdapter<_>>(
            &command_ack_topic,
            None,
        )
        .unwrap();

    // initialize publisher service
    let mut p = PublisherBase::new(command_sender, command_ack_receiver);

    // initialization delay
    sleep(Duration::from_secs(5));

    println!("Running Publisher");
    while !p.check_completed() {
        p.cycle();
    }

    println!("Publisher completed running successfully!");
}
