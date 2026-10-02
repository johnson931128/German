use std::collections::HashMap;
use std::fmt;

use super::{Record, Topic};

#[derive(Debug, PartialEq, Eq)]
pub enum ModelError {
    TopicAlreadyExists(String),
    TopicNotFound(String),
}

impl fmt::Display for ModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopicAlreadyExists(name) => write!(formatter, "topic already exists: {name}"),
            Self::TopicNotFound(name) => write!(formatter, "topic not found: {name}"),
        }
    }
}

impl std::error::Error for ModelError {}

#[derive(Debug, Default)]
pub struct Broker {
    topics: HashMap<String, Topic>,
}

impl Broker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_topic(&mut self, name: &str) -> Result<(), ModelError> {
        if self.topics.contains_key(name) {
            return Err(ModelError::TopicAlreadyExists(name.to_owned()));
        }

        self.topics.insert(name.to_owned(), Topic::new());
        Ok(())
    }

    pub(crate) fn publish(&mut self, topic_name: &str, record: Record) -> Result<(), ModelError> {
        let topic = self
            .topics
            .get_mut(topic_name)
            .ok_or_else(|| ModelError::TopicNotFound(topic_name.to_owned()))?;
        topic.store(record);
        Ok(())
    }

    pub(crate) fn records(&self, topic_name: &str) -> Result<&[Record], ModelError> {
        self.topics
            .get(topic_name)
            .map(Topic::records)
            .ok_or_else(|| ModelError::TopicNotFound(topic_name.to_owned()))
    }
}
