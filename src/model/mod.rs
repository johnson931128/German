mod broker;
mod consumer;
mod producer;
mod record;
mod topic;

pub use broker::{Broker, ModelError};
pub use consumer::Consumer;
pub use producer::Producer;
pub use record::Record;
pub use topic::Topic;

#[cfg(test)]
mod tests {
    use super::{Broker, Consumer, ModelError, Producer};

    #[test]
    fn broker_creates_topic() {
        let mut broker = Broker::new();

        broker.create_topic("temperature").unwrap();

        assert!(Consumer.consume(&broker, "temperature").unwrap().is_empty());
    }

    #[test]
    fn producer_publishes_record_to_topic() {
        let mut broker = Broker::new();
        broker.create_topic("temperature").unwrap();

        Producer.publish(&mut broker, "temperature", "30").unwrap();

        let records = Consumer.consume(&broker, "temperature").unwrap();
        assert_eq!(records[0].value, "30");
    }

    #[test]
    fn records_keep_publish_order() {
        let mut broker = Broker::new();
        broker.create_topic("temperature").unwrap();
        let producer = Producer;

        for value in ["30", "31", "29", "32"] {
            producer.publish(&mut broker, "temperature", value).unwrap();
        }

        let values: Vec<_> = Consumer
            .consume(&broker, "temperature")
            .unwrap()
            .iter()
            .map(|record| record.value.as_str())
            .collect();
        assert_eq!(values, ["30", "31", "29", "32"]);
    }

    #[test]
    fn consumer_reads_topic_records() {
        let mut broker = Broker::new();
        broker.create_topic("temperature").unwrap();
        Producer.publish(&mut broker, "temperature", "30").unwrap();

        let records = Consumer.consume(&broker, "temperature").unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].value, "30");
    }

    #[test]
    fn missing_topic_operations_return_errors() {
        let mut broker = Broker::new();

        assert_eq!(
            Producer.publish(&mut broker, "missing", "30"),
            Err(ModelError::TopicNotFound("missing".to_owned()))
        );
        assert_eq!(
            Consumer.consume(&broker, "missing"),
            Err(ModelError::TopicNotFound("missing".to_owned()))
        );
    }

    #[test]
    fn broker_rejects_duplicate_topic() {
        let mut broker = Broker::new();
        broker.create_topic("temperature").unwrap();

        assert_eq!(
            broker.create_topic("temperature"),
            Err(ModelError::TopicAlreadyExists("temperature".to_owned()))
        );
    }
}
