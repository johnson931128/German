use std::time::{Duration, Instant};

use eframe::egui;

use crate::simulation::{Node, Simulation};

use super::canvas;

pub struct PubSubApp {
    simulation: Simulation,
    selected_node: Option<Node>,
    last_frame: Instant,
}

impl PubSubApp {
    pub fn new() -> Self {
        Self {
            simulation: Simulation::new(),
            selected_node: None,
            last_frame: Instant::now(),
        }
    }

    fn controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Play").clicked() {
                self.simulation.play();
            }
            if ui.button("Pause").clicked() {
                self.simulation.pause();
            }
            if ui.button("Step").clicked() {
                self.simulation.step();
            }
            if ui.button("Reset").clicked() {
                self.simulation.reset();
                self.selected_node = None;
            }
            ui.separator();
            ui.label("Speed");
            egui::ComboBox::from_id_salt("simulation_speed")
                .selected_text(format!("{}x", self.simulation.speed))
                .width(65.0)
                .show_ui(ui, |ui| {
                    for speed in [0.5, 1.0, 2.0] {
                        ui.selectable_value(&mut self.simulation.speed, speed, format!("{speed}x"));
                    }
                });
            ui.label(format!("t = {:.2}s", self.simulation.current_time));
            ui.weak(if self.simulation.remaining_events() == 0 {
                "Finished"
            } else if self.simulation.running {
                "Playing"
            } else {
                "Paused"
            });
        });
    }

    fn node_details(&self, ui: &mut egui::Ui) {
        match self.selected_node {
            Some(Node::Producer) => {
                let last = self
                    .simulation
                    .last_generated
                    .as_ref()
                    .map_or("none", |record| record.value.as_str());
                ui.label(format!(
                    "Producer | Generated: {} | Last Record: {last}",
                    self.simulation.generated_count
                ));
            }
            Some(Node::Broker) => {
                let stored: usize = self
                    .simulation
                    .topics()
                    .map(|(_, topic)| topic.records().len())
                    .sum();
                ui.label(format!(
                    "Broker | Received: {} | Topics: {} | Stored: {stored}",
                    self.simulation.broker_received_count,
                    self.simulation.topics().count()
                ));
            }
            Some(Node::Consumer) => {
                let values: Vec<_> = self
                    .simulation
                    .consumer_records
                    .iter()
                    .map(|record| record.value.as_str())
                    .collect();
                ui.label(format!(
                    "Consumer | Received: {} | Records: [{}]",
                    values.len(),
                    values.join(", ")
                ));
            }
            None => {
                ui.weak("Click a node to inspect its current state. Step executes one event.");
            }
        }
    }
}

impl eframe::App for PubSubApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_frame).as_secs_f64();
        self.last_frame = now;
        self.simulation.advance(elapsed);

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Pub/Sub Simulation");
                ui.weak("In-memory model / simulated time");
            });
            let canvas_height = (ui.available_height() - 175.0).max(180.0);
            canvas::show(ui, &self.simulation, &mut self.selected_node, canvas_height);
            ui.separator();
            self.controls(ui);
            self.node_details(ui);
            ui.label(egui::RichText::new("Event log").strong());
            egui::ScrollArea::vertical()
                .id_salt("event_log")
                .max_height(90.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    if self.simulation.log.is_empty() {
                        ui.weak("Ready. Play or Step to begin the temperature scenario.");
                    }
                    for event in &self.simulation.log {
                        ui.monospace(format!("[{:.1}] {}", event.time, event.description));
                    }
                });
        });

        if self.simulation.running {
            ui.ctx().request_repaint_after(Duration::from_millis(16));
        }
    }
}
