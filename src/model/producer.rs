use super::{Broker, ModelError, Record};

#[derive(Debug, Default, Clone, Copy)]
pub struct Producer;

impl Producer {
    pub fn publish(
        &self,
        broker: &mut Broker,
        topic: &str,
        value: impl Into<String>,
    ) -> Result<(), ModelError> {
        broker.publish(topic, Record::new(value))
    }
}
