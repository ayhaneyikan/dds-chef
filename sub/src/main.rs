use messages::SimpleMessage;
use rustdds::{DomainParticipant, QosPolicyBuilder, policy, TopicKind, CDRDeserializerAdapter};


fn main() {
  // always necessary establishing a common dds domain (i think)
  let domain_participant = DomainParticipant::new(0).unwrap();
  
  // some kind of reliability declaration
  //  sets no blocking for responses
  // builds the policy?
  let qos = QosPolicyBuilder::new()
    .reliability(policy::Reliability::Reliable { max_blocking_time: rustdds::Duration::DURATION_ZERO })
    .build();
  
  // DDS Subscriber, only one is necessary for each thread (slight difference to DDS specification)
  let subscriber = domain_participant.create_subscriber(&qos).unwrap();

  // Some DDS Topic that we can write and read from (basically only binds readers and writers together)
  let some_topic = domain_participant.create_topic("simple_test".to_string(), "SimpleMessage".to_string(), &qos, TopicKind::NoKey).unwrap();

  // create data writer instance which will publish a message to the given topic
  // Creating DataReader requires type and deserializer adapter (which is recommended to be CDR).
  // Reader needs to be mutable if any operations are used.
  let mut reader = subscriber
    .create_datareader_no_key::<SimpleMessage, CDRDeserializerAdapter<SimpleMessage>>(&some_topic, None)
    .unwrap();

  // iterate until invalid
  while let Ok(Some(sample)) = reader.take_next_sample() {
    let msg = sample.value();
    dbg!(msg);
  }
}
