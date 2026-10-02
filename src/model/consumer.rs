use super::{Broker, ModelError, Record};

#[derive(Debug, Default, Clone, Copy)]
pub struct Consumer;

impl Consumer {
    pub fn consume<'a>(&self, broker: &'a Broker, topic: &str) -> Result<&'a [Record], ModelError> {
        broker.records(topic)
    }
}
