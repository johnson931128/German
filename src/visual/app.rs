use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, RichText};

use crate::model::{Broker, Consumer, ModelError, Producer, Record};

const FEEDBACK_DURATION: Duration = Duration::from_secs(3);

pub struct PubSubApp {
    broker: Broker,
    producer: Producer,
    consumer: Consumer,
    producer_topic: String,
    producer_value: String,
    new_topic_name: String,
    consumer_topic: String,
    consumed_records: Vec<Record>,
    feedback: Option<(String, Instant)>,
}

impl PubSubApp {
    pub fn new() -> Self {
        let mut broker = Broker::new();
        broker
            .create_topic("temperature")
            .expect("initial topic name is unique");
        broker
            .create_topic("orders")
            .expect("initial topic name is unique");

        Self {
            broker,
            producer: Producer,
            consumer: Consumer,
            producer_topic: "temperature".to_owned(),
            producer_value: "30".to_owned(),
            new_topic_name: String::new(),
            consumer_topic: "temperature".to_owned(),
            consumed_records: Vec::new(),
            feedback: None,
        }
    }

    fn topic_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self
            .broker
            .topics()
            .map(|(name, _topic)| name.to_owned())
            .collect();
        names.sort();
        names
    }

    fn show_model_error(&mut self, error: ModelError) {
        self.feedback = Some((format!("ModelError: {error}"), Instant::now()));
    }

    fn show_flow(&mut self, description: String) {
        self.feedback = Some((description, Instant::now()));
    }

    fn producer_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading("Producer");
            ui.label("Send a Record to the Broker");
            ui.add_space(14.0);

            ui.label("Target Topic");
            let names = self.topic_names();
            egui::ComboBox::from_id_salt("producer_topic")
                .selected_text(&self.producer_topic)
                .show_ui(ui, |ui| {
                    for name in &names {
                        ui.selectable_value(&mut self.producer_topic, name.clone(), name);
                    }
                });

            ui.add_space(10.0);
            ui.label("Record value");
            ui.text_edit_singleline(&mut self.producer_value);

            ui.add_space(8.0);
            if ui.button("Publish Record →").clicked() {
                match self.producer.publish(
                    &mut self.broker,
                    &self.producer_topic,
                    self.producer_value.clone(),
                ) {
                    Ok(()) => self.show_flow(format!(
                        "Producer → Broker: stored \"{}\" in {}",
                        self.producer_value, self.producer_topic
                    )),
                    Err(error) => self.show_model_error(error),
                }
            }

            ui.separator();
            ui.heading("Create Topic");
            ui.label("Broker manages Topics inside its boundary.");
            ui.text_edit_singleline(&mut self.new_topic_name);
            if ui.button("Create Topic").clicked() {
                match self.broker.create_topic(self.new_topic_name.trim()) {
                    Ok(()) => {
                        self.show_flow(format!(
                            "Broker created Topic: {}",
                            self.new_topic_name.trim()
                        ));
                        self.producer_topic = self.new_topic_name.trim().to_owned();
                        self.consumer_topic = self.new_topic_name.trim().to_owned();
                        self.new_topic_name.clear();
                    }
                    Err(error) => self.show_model_error(error),
                }
            }
        });
    }

    fn consumer_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading("Consumer");
            ui.label("Read all current Records from a Topic");
            ui.add_space(14.0);

            ui.label("Source Topic");
            let names = self.topic_names();
            egui::ComboBox::from_id_salt("consumer_topic")
                .selected_text(&self.consumer_topic)
                .show_ui(ui, |ui| {
                    for name in &names {
                        ui.selectable_value(&mut self.consumer_topic, name.clone(), name);
                    }
                });

            ui.add_space(8.0);
            if ui.button("← Consume Records").clicked() {
                match self.consumer.consume(&self.broker, &self.consumer_topic) {
                    Ok(records) => {
                        self.consumed_records = records.to_vec();
                        self.show_flow(format!(
                            "Broker / {} → Consumer: read {} Record(s)",
                            self.consumer_topic,
                            self.consumed_records.len()
                        ));
                    }
                    Err(error) => self.show_model_error(error),
                }
            }

            ui.separator();
            ui.heading("Recently consumed");
            if self.consumed_records.is_empty() {
                ui.weak("No Records read yet");
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (index, record) in self.consumed_records.iter().enumerate() {
                        ui.label(format!("{}. {}", index + 1, record.value));
                    }
                });
            }
        });
    }

    fn broker_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(28, 39, 54))
            .stroke(egui::Stroke::new(2.0, Color32::from_rgb(89, 157, 220)))
            .inner_margin(egui::Margin::same(18))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(RichText::new("Broker").size(26.0));
                    ui.label(RichText::new("Server · owns and manages Topics").weak());
                });
                ui.add_space(12.0);

                let mut topics: Vec<_> = self
                    .broker
                    .topics()
                    .map(|(name, topic)| (name.to_owned(), topic))
                    .collect();
                topics.sort_by(|left, right| left.0.cmp(&right.0));

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (name, topic) in topics {
                        egui::Frame::group(ui.style())
                            .fill(Color32::from_rgb(37, 51, 69))
                            .inner_margin(egui::Margin::same(12))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!("Topic: {name}")).strong().size(18.0),
                                    );
                                    ui.label(
                                        RichText::new(format!(
                                            "{} Record(s)",
                                            topic.records().len()
                                        ))
                                        .weak(),
                                    );
                                });
                                ui.add_space(6.0);
                                if topic.records().is_empty() {
                                    ui.weak("empty");
                                } else {
                                    ui.horizontal_wrapped(|ui| {
                                        for (index, record) in topic.records().iter().enumerate() {
                                            ui.label(format!("[{}]", record.value));
                                            if index + 1 < topic.records().len() {
                                                ui.label("→");
                                            }
                                        }
                                    });
                                }
                            });
                        ui.add_space(10.0);
                    }
                });
            });
    }
}

impl eframe::App for PubSubApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ui.vertical(|ui| {
            ui.horizontal_centered(|ui| {
                ui.heading("Producer");
                ui.label("→ sends Record to →");
                ui.heading(RichText::new("Broker").color(Color32::LIGHT_BLUE));
                ui.label("→ serves Topic Records to →");
                ui.heading("Consumer");
            });
            if let Some((message, created_at)) = &self.feedback {
                if created_at.elapsed() < FEEDBACK_DURATION {
                    ui.label(RichText::new(message).color(Color32::LIGHT_GREEN));
                    ctx.request_repaint_after(Duration::from_millis(100));
                } else {
                    self.feedback = None;
                }
            }
            ui.separator();

            let available = ui.available_size();
            let left_width = (available.x * 0.23).max(190.0);
            let right_width = (available.x * 0.24).max(205.0);
            let broker_width = (available.x - left_width - right_width - 16.0).max(300.0);
            let panel_height = available.y.max(420.0);

            ui.horizontal_top(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(left_width, panel_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| self.producer_panel(ui),
                );
                ui.allocate_ui_with_layout(
                    egui::vec2(broker_width, panel_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| self.broker_panel(ui),
                );
                ui.allocate_ui_with_layout(
                    egui::vec2(right_width, panel_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| self.consumer_panel(ui),
                );
            });
        });
    }
}
