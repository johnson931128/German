#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    Producer,
    Broker,
    Consumer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationEvent {
    ProduceRecord(String),
    SendToBroker(String),
    BrokerReceive(String),
    StoreRecord(String),
    ConsumerRequest,
    SendToConsumer(String),
    ConsumerReceive(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessagePath {
    ToBroker,
    IntoTopic,
    ToConsumer,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub value: String,
    pub source: Node,
    pub destination: Node,
    pub path: MessagePath,
    pub progress: f64,
    pub start_time: f64,
    pub duration: f64,
}

impl Message {
    pub fn update_progress(&mut self, time: f64) {
        self.progress = ((time - self.start_time) / self.duration).clamp(0.0, 1.0);
    }
}
