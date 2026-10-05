use std::collections::VecDeque;

use crate::model::{Broker, Consumer, Producer, Record, Topic};

use super::{Message, MessagePath, Node, SimulationEvent};

const TOPIC: &str = "temperature";

#[derive(Debug, Clone)]
struct ScheduledEvent {
    at: f64,
    event: SimulationEvent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub time: f64,
    pub description: String,
}

/// A local, deterministic timeline, independent of egui and wall-clock time.
pub struct Simulation {
    pub current_time: f64,
    pub running: bool,
    pub speed: f64,
    pub active_messages: Vec<Message>,
    pub log: Vec<LogEntry>,
    pub generated_count: usize,
    pub last_generated: Option<Record>,
    pub broker_received_count: usize,
    pub consumer_records: Vec<Record>,
    broker: Broker,
    producer: Producer,
    consumer: Consumer,
    events: VecDeque<ScheduledEvent>,
}

impl Simulation {
    pub fn new() -> Self {
        let mut broker = Broker::new();
        broker
            .create_topic(TOPIC)
            .expect("scenario topic is unique");
        let mut simulation = Self {
            current_time: 0.0,
            running: false,
            speed: 1.0,
            active_messages: Vec::new(),
            log: Vec::new(),
            generated_count: 0,
            last_generated: None,
            broker_received_count: 0,
            consumer_records: Vec::new(),
            broker,
            producer: Producer,
            consumer: Consumer,
            events: VecDeque::new(),
        };

        for (index, value) in ["30", "31", "29", "32"].iter().enumerate() {
            let start = index as f64 * 2.0;
            simulation.schedule(start, SimulationEvent::ProduceRecord((*value).to_owned()));
            simulation.schedule(
                start + 0.5,
                SimulationEvent::SendToBroker((*value).to_owned()),
            );
            simulation.schedule(
                start + 1.0,
                SimulationEvent::BrokerReceive((*value).to_owned()),
            );
            simulation.schedule(
                start + 1.2,
                SimulationEvent::StoreRecord((*value).to_owned()),
            );
        }
        simulation.schedule(8.0, SimulationEvent::ConsumerRequest);
        simulation
    }

    pub fn topics(&self) -> impl Iterator<Item = (&str, &Topic)> {
        self.broker.topics()
    }

    pub fn remaining_events(&self) -> usize {
        self.events.len()
    }

    pub fn play(&mut self) {
        self.running = !self.events.is_empty();
    }

    pub fn pause(&mut self) {
        self.running = false;
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Each click advances to exactly one event, including events at the same time.
    pub fn step(&mut self) {
        self.pause();
        self.execute_next_event();
    }

    pub fn advance(&mut self, elapsed_seconds: f64) {
        if !self.running || !elapsed_seconds.is_finite() || elapsed_seconds <= 0.0 {
            return;
        }
        let target = self.current_time + elapsed_seconds * self.speed;
        while self.events.front().is_some_and(|event| event.at <= target) {
            self.execute_next_event();
        }
        if self.events.is_empty() {
            self.pause();
        } else {
            self.current_time = target;
            self.update_messages();
        }
    }

    fn schedule(&mut self, at: f64, event: SimulationEvent) {
        let position = self.events.iter().position(|queued| queued.at > at);
        let scheduled = ScheduledEvent { at, event };
        if let Some(position) = position {
            self.events.insert(position, scheduled);
        } else {
            self.events.push_back(scheduled);
        }
    }

    fn update_messages(&mut self) {
        for message in &mut self.active_messages {
            message.update_progress(self.current_time);
        }
    }

    fn add_message(&mut self, value: String, path: MessagePath, duration: f64) {
        let (source, destination) = match path {
            MessagePath::ToBroker => (Node::Producer, Node::Broker),
            MessagePath::IntoTopic => (Node::Broker, Node::Broker),
            MessagePath::ToConsumer => (Node::Broker, Node::Consumer),
        };
        self.active_messages.push(Message {
            value,
            source,
            destination,
            path,
            progress: 0.0,
            start_time: self.current_time,
            duration,
        });
    }

    fn remove_message(&mut self, value: &str, path: MessagePath) {
        self.active_messages
            .retain(|message| message.value != value || message.path != path);
    }

    fn execute_next_event(&mut self) {
        let Some(scheduled) = self.events.pop_front() else {
            return;
        };
        self.current_time = scheduled.at;
        self.update_messages();
        let description = match scheduled.event {
            SimulationEvent::ProduceRecord(value) => {
                self.generated_count += 1;
                self.last_generated = Some(Record::new(value.clone()));
                format!("Producer created Record \"{value}\"")
            }
            SimulationEvent::SendToBroker(value) => {
                self.add_message(value.clone(), MessagePath::ToBroker, 0.5);
                format!("Record \"{value}\" sent to Broker (Topic {TOPIC})")
            }
            SimulationEvent::BrokerReceive(value) => {
                self.remove_message(&value, MessagePath::ToBroker);
                self.broker_received_count += 1;
                self.add_message(value.clone(), MessagePath::IntoTopic, 0.2);
                format!("Broker received Record \"{value}\"")
            }
            SimulationEvent::StoreRecord(value) => {
                self.remove_message(&value, MessagePath::IntoTopic);
                match self
                    .producer
                    .publish(&mut self.broker, TOPIC, value.clone())
                {
                    Ok(()) => format!("Broker stored Record \"{value}\" in Topic {TOPIC}"),
                    Err(error) => format!("ModelError: {error}"),
                }
            }
            SimulationEvent::ConsumerRequest => {
                // The Consumer reads the actual model. Delivery is visualized separately.
                match self.consumer.consume(&self.broker, TOPIC) {
                    Ok(records) => {
                        let records = records.to_vec();
                        for (index, record) in records.into_iter().enumerate() {
                            let send_time = self.current_time + 0.5 + index as f64 * 0.6;
                            self.schedule(
                                send_time,
                                SimulationEvent::SendToConsumer(record.value.clone()),
                            );
                            self.schedule(
                                send_time + 0.8,
                                SimulationEvent::ConsumerReceive(record.value),
                            );
                        }
                        format!("Consumer requested Topic {TOPIC}")
                    }
                    Err(error) => format!("ModelError: {error}"),
                }
            }
            SimulationEvent::SendToConsumer(value) => {
                self.add_message(value.clone(), MessagePath::ToConsumer, 0.8);
                format!("Broker sent Record \"{value}\" to Consumer")
            }
            SimulationEvent::ConsumerReceive(value) => {
                self.remove_message(&value, MessagePath::ToConsumer);
                self.consumer_records.push(Record::new(value.clone()));
                format!("Consumer received Record \"{value}\"")
            }
        };
        self.log.push(LogEntry {
            time: self.current_time,
            description,
        });
        if self.events.is_empty() {
            self.pause();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored(simulation: &Simulation) -> Vec<&str> {
        simulation
            .topics()
            .find(|(name, _)| *name == TOPIC)
            .unwrap()
            .1
            .records()
            .iter()
            .map(|record| record.value.as_str())
            .collect()
    }

    #[test]
    fn step_executes_one_event_and_only_store_changes_topic() {
        let mut simulation = Simulation::new();
        for expected in 1..=3 {
            simulation.step();
            assert_eq!(simulation.log.len(), expected);
            assert!(stored(&simulation).is_empty());
        }
        assert_eq!(simulation.broker_received_count, 1);
        simulation.step();
        assert_eq!(simulation.log.len(), 4);
        assert_eq!(stored(&simulation), ["30"]);
        assert!(!simulation.running);
    }

    #[test]
    fn pause_freezes_time_and_messages_and_speed_scales_time() {
        let mut simulation = Simulation::new();
        simulation.step();
        simulation.step();
        simulation.speed = 0.5;
        simulation.play();
        simulation.advance(0.5);
        assert_eq!(simulation.current_time, 0.75);
        assert_eq!(simulation.active_messages[0].progress, 0.5);
        simulation.pause();
        simulation.advance(10.0);
        assert_eq!(simulation.current_time, 0.75);
        assert_eq!(simulation.active_messages[0].progress, 0.5);
        simulation.speed = 2.0;
        simulation.play();
        simulation.advance(0.1);
        assert!((simulation.current_time - 0.95).abs() < 1e-9);
        assert!((simulation.active_messages[0].progress - 0.9).abs() < 1e-9);
    }

    #[test]
    fn playback_and_single_event_steps_produce_identical_results() {
        let mut stepped = Simulation::new();
        while stepped.remaining_events() > 0 {
            stepped.step();
        }
        let mut played = Simulation::new();
        played.play();
        played.advance(20.0);
        assert_eq!(played.log, stepped.log);
        assert_eq!(stored(&played), ["30", "31", "29", "32"]);
        assert_eq!(played.consumer_records, stepped.consumer_records);
        assert_eq!(played.consumer_records.len(), 4);
        let consumed: Vec<_> = played
            .consumer_records
            .iter()
            .map(|record| record.value.as_str())
            .collect();
        assert_eq!(consumed, ["30", "31", "29", "32"]);
        assert_eq!(played.generated_count, 4);
        assert_eq!(played.broker_received_count, 4);
        assert!(played.active_messages.is_empty());
        assert!(!played.running);
    }

    #[test]
    fn reset_restores_empty_initial_state() {
        let mut simulation = Simulation::new();
        simulation.play();
        simulation.advance(20.0);
        simulation.reset();
        assert_eq!(simulation.current_time, 0.0);
        assert_eq!(simulation.remaining_events(), 17);
        assert!(stored(&simulation).is_empty());
        assert!(simulation.consumer_records.is_empty());
        assert!(simulation.log.is_empty());
        assert!(simulation.active_messages.is_empty());
        assert_eq!(simulation.generated_count, 0);
        assert!(!simulation.running);
        assert_eq!(simulation.speed, 1.0);
    }

    #[test]
    fn consumer_delivery_follows_request_and_preserves_broker_records() {
        let mut simulation = Simulation::new();
        simulation.play();
        simulation.advance(8.75);
        assert_eq!(stored(&simulation), ["30", "31", "29", "32"]);
        assert!(simulation.consumer_records.is_empty());
        assert_eq!(simulation.active_messages[0].path, MessagePath::ToConsumer);
        assert!((simulation.active_messages[0].progress - 0.3125).abs() < 1e-9);
        simulation.advance(0.6);
        assert_eq!(simulation.consumer_records.len(), 1);
        assert_eq!(simulation.consumer_records[0].value, "30");
        assert_eq!(stored(&simulation), ["30", "31", "29", "32"]);
    }
}
