use rustdds::{
    dds::ReadError,
    no_key::{DataReader, DataWriter},
    CDRDeserializerAdapter, CDRSerializerAdapter, DomainParticipant, QosPolicyBuilder, TopicKind,
};
use serde::Serialize;
use std::fmt::Debug;

/// A sender instance for sending some message type T
pub struct Sender<T>
where
    T: Debug + Serialize,
{
    writer: DataWriter<T>,
}

impl<T> Sender<T>
where
    T: Debug + Serialize,
{
    pub fn new(domain_id: u16, topic_name: String, topic_desc: Option<String>) -> Self {
        // create domain participant and qos policies
        let participant = DomainParticipant::new(domain_id).unwrap();
        let qos = QosPolicyBuilder::new().build();

        // create topic
        let topic = participant
            .create_topic(
                topic_name.clone(),
                topic_desc.unwrap_or(topic_name),
                &qos,
                TopicKind::NoKey,
            )
            .unwrap();

        // create publisher and data writer
        let publisher = participant.create_publisher(&qos).unwrap();

        Self {
            writer: publisher
                .create_datawriter_no_key::<T, CDRSerializerAdapter<_>>(&topic, None)
                .unwrap(),
        }
    }

    pub fn send(&self, msg: T) {
        self.writer.write(msg, None).unwrap();
    }
}

/// A receiver instance for receiving some message type T
pub struct Receiver<T>
where
    T: 'static + Debug + for<'de> serde::Deserialize<'de>,
{
    reader: DataReader<T>,
}

impl<T> Receiver<T>
where
    T: 'static + Debug + for<'de> serde::Deserialize<'de>,
{
    pub fn new(domain_id: u16, topic_name: String, topic_desc: Option<String>) -> Self {
        // create domain participant and qos policies
        let participant = DomainParticipant::new(domain_id).unwrap();
        let qos = QosPolicyBuilder::new().build();

        // create topic
        let topic = participant
            .create_topic(
                topic_name.clone(),
                topic_desc.unwrap_or(topic_name),
                &qos,
                TopicKind::NoKey,
            )
            .unwrap();

        // create subscriber
        let subscriber = participant.create_subscriber(&qos).unwrap();

        Self {
            reader: subscriber
                .create_datareader_no_key::<T, CDRDeserializerAdapter<_>>(&topic, None)
                .unwrap(),
        }
    }

    pub fn receive(&mut self) -> Result<Option<T>, ReadError> {
        match self.reader.take_next_sample() {
            Ok(Some(msg)) => Ok(Some(msg.into_value())),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
