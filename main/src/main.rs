use messages::SimpleMessage;
use rustdds::{DomainParticipant, QosPolicyBuilder, policy, TopicKind, CDRSerializerAdapter};


fn main() {
  // always necessary establishing a common dds domain (i think)
  let domain_participant = DomainParticipant::new(0).unwrap();
  
  // some kind of reliability declaration
  //  sets no blocking for responses
  // builds the policy?
  let qos = QosPolicyBuilder::new()
    .reliability(policy::Reliability::Reliable { max_blocking_time: rustdds::Duration::DURATION_ZERO })
    .build();
  
  // create publisher instance
  // DDS Publisher, only one is necessary for each thread (slight difference to DDS specification)
  let publisher = domain_participant.create_publisher(&qos).unwrap();

  // Some DDS Topic that we can write and read from (basically only binds readers and writers together)
  let some_topic = domain_participant.create_topic("simple_test".to_string(), "SimpleMessage".to_string(), &qos, TopicKind::NoKey).unwrap();

  // create data writer instance which will publish a message to the given topic
  // Creating DataWriter required type and serializer adapter (which is recommended to be CDR).
  // I AM UNSURE WHY THE QOS IS NOT PASSED IN HERE, PERHAPS IT IS UNNECESSARY AS IT HAS BEEN PASSED INTO THE ENTIRE TOPIC
  let writer = publisher
    .create_datawriter_no_key::<SimpleMessage, CDRSerializerAdapter<SimpleMessage>>(&some_topic, None)
    .unwrap();

  // create message to be sent  
  let msg = SimpleMessage::new(67, 10.20);

  // send message to all who listen to the topic
  match writer.write(msg, None) {
    Ok(_) => println!("Message published successfully"),
    Err(_) => println!("Message failed to publish :("),
  }
}
