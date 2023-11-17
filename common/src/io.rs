use rustdds::{
    no_key::{DataReader, DataWriter},
    CDRDeserializerAdapter, CDRSerializerAdapter, DomainParticipant, Publisher, QosPolicies,
    QosPolicyBuilder, Subscriber, Topic, TopicKind,
};
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

/// Constant id representing the DDS domain
const DOMAIN_ID: u16 = 0;

/// A sender instance for sending messages of type T
pub struct Sender<T>
where
    T: Debug + Serialize,
{
    _participant: DomainParticipant,
    _qos: QosPolicies,
    _topic: Topic,
    _publisher: Publisher,
    writer: DataWriter<T>,
}

impl<T> Sender<T>
where
    T: Debug + Serialize,
{
    /// Creates new Sender within given domain for a given topic
    ///
    /// Handles creation of DDS components and maintains internally:
    /// - Participant
    /// - Topic
    /// - Publisher
    /// - DataWriter
    pub fn new(topic_name: String, topic_desc: Option<String>) -> Self {
        // create domain participant and qos policies
        let participant = DomainParticipant::new(DOMAIN_ID).unwrap();
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

        let writer = publisher
            .create_datawriter_no_key::<T, CDRSerializerAdapter<_>>(&topic, None)
            .unwrap();

        Self {
            _participant: participant,
            _qos: qos,
            _topic: topic,
            _publisher: publisher,
            writer,
        }
    }

    /// Sends the given message via DDS to this Sender's topic.
    pub fn send(&self, msg: T) {
        self.writer.write(msg, None).unwrap();
    }
}

/// A receiver instance for receiving messages of type T
pub struct Receiver<T>
where
    T: 'static + Debug + for<'de> Deserialize<'de>,
{
    _participant: DomainParticipant,
    _qos: QosPolicies,
    _topic: Topic,
    _subscriber: Subscriber,
    reader: DataReader<T>,
}

impl<T> Receiver<T>
where
    T: 'static + Debug + for<'de> Deserialize<'de>,
{
    /// Creates new Receiver within given domain for a given topic
    ///
    /// Handles creation of DDS components and maintains internally:
    /// - Participant
    /// - Topic
    /// - Subscriber
    /// - DataReader
    pub fn new(topic_name: String, topic_desc: Option<String>) -> Self {
        // create domain participant and qos policies
        let participant = DomainParticipant::new(DOMAIN_ID).unwrap();
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

        let reader = subscriber
            .create_datareader_no_key::<T, CDRDeserializerAdapter<_>>(&topic, None)
            .unwrap();

        Self {
            _participant: participant,
            _qos: qos,
            _topic: topic,
            _subscriber: subscriber,
            reader,
        }
    }

    /// Attempts to receive a given message via DDS from this Receiver's topic.
    pub fn receive(&mut self) -> Option<T> {
        match self.reader.take_next_sample() {
            Ok(Some(msg)) => Some(msg.into_value()),
            _ => None,
        }
    }
}
