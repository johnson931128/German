use eframe::egui::{self, Align2, Color32, FontId, Painter, Pos2, Rect, Stroke, StrokeKind};

use crate::simulation::{MessagePath, Node, Simulation};

const TEXT: Color32 = Color32::from_rgb(226, 234, 245);
const MUTED: Color32 = Color32::from_rgb(149, 168, 190);
const ORANGE: Color32 = Color32::from_rgb(251, 177, 69);
const BLUE: Color32 = Color32::from_rgb(90, 190, 231);

fn label(painter: &Painter, position: Pos2, text: &str, size: f32, color: Color32) {
    painter.text(
        position,
        Align2::LEFT_TOP,
        text,
        FontId::proportional(size),
        color,
    );
}

fn node(painter: &Painter, rect: Rect, selected: bool, color: Color32) {
    painter.rect(
        rect,
        12,
        Color32::from_rgb(28, 39, 54),
        Stroke::new(
            if selected { 3.0 } else { 1.5 },
            if selected { ORANGE } else { color },
        ),
        StrokeKind::Inside,
    );
}

fn record_position(topic: Rect, index: usize) -> Pos2 {
    let columns = ((topic.width() - 24.0) / 48.0).floor().max(1.0) as usize;
    topic.left_top()
        + egui::vec2(
            32.0 + (index % columns) as f32 * 48.0,
            64.0 + (index / columns) as f32 * 34.0,
        )
}

fn token(painter: &Painter, center: Pos2, value: &str, color: Color32) {
    let rect = Rect::from_center_size(center, egui::vec2(40.0, 28.0));
    painter.rect_filled(rect, 5, color);
    painter.text(
        center,
        Align2::CENTER_CENTER,
        value,
        FontId::monospace(16.0),
        Color32::BLACK,
    );
}

pub fn show(ui: &mut egui::Ui, simulation: &Simulation, selected: &mut Option<Node>, height: f32) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 8, Color32::from_rgb(16, 23, 33));

    let width = rect.width();
    let center_y = rect.center().y;
    let producer = Rect::from_center_size(
        Pos2::new(rect.left() + width * 0.14, center_y),
        egui::vec2(width * 0.20, 128.0),
    );
    let broker = Rect::from_center_size(
        Pos2::new(rect.left() + width * 0.50, center_y),
        egui::vec2(width * 0.38, height.min(280.0) - 24.0),
    );
    let consumer = Rect::from_center_size(
        Pos2::new(rect.left() + width * 0.86, center_y),
        egui::vec2(width * 0.20, 192.0),
    );
    let topic = Rect::from_min_max(
        broker.left_top() + egui::vec2(14.0, 70.0),
        broker.right_bottom() - egui::vec2(14.0, 14.0),
    );

    painter.arrow(
        producer.right_center(),
        broker.left_center() - producer.right_center(),
        Stroke::new(2.0, ORANGE),
    );
    painter.arrow(
        broker.right_center(),
        consumer.left_center() - broker.right_center(),
        Stroke::new(2.0, BLUE),
    );

    node(
        &painter,
        producer,
        *selected == Some(Node::Producer),
        ORANGE,
    );
    node(&painter, broker, *selected == Some(Node::Broker), BLUE);
    node(&painter, consumer, *selected == Some(Node::Consumer), BLUE);
    label(
        &painter,
        producer.left_top() + egui::vec2(12.0, 12.0),
        "Producer",
        20.0,
        TEXT,
    );
    label(
        &painter,
        producer.left_top() + egui::vec2(12.0, 48.0),
        &format!("Generated: {}", simulation.generated_count),
        14.0,
        MUTED,
    );
    let last = simulation
        .last_generated
        .as_ref()
        .map_or("-", |record| record.value.as_str());
    label(
        &painter,
        producer.left_top() + egui::vec2(12.0, 74.0),
        &format!("Last: {last}"),
        16.0,
        TEXT,
    );

    label(
        &painter,
        broker.left_top() + egui::vec2(14.0, 12.0),
        "Broker",
        23.0,
        TEXT,
    );
    label(
        &painter,
        broker.left_top() + egui::vec2(14.0, 42.0),
        &format!("Received: {}", simulation.broker_received_count),
        14.0,
        MUTED,
    );
    painter.rect(
        topic,
        6,
        Color32::from_rgb(35, 51, 71),
        Stroke::new(1.0, BLUE),
        StrokeKind::Inside,
    );
    let mut stored_count = 0;
    for (name, state) in simulation.topics() {
        label(
            &painter,
            topic.left_top() + egui::vec2(12.0, 10.0),
            name,
            18.0,
            TEXT,
        );
        stored_count = state.records().len();
        label(
            &painter,
            topic.left_top() + egui::vec2(12.0, 34.0),
            &format!("Topic / {stored_count} stored"),
            13.0,
            MUTED,
        );
        for (index, record) in state.records().iter().enumerate() {
            token(
                &painter,
                record_position(topic, index),
                &record.value,
                ORANGE,
            );
        }
    }

    label(
        &painter,
        consumer.left_top() + egui::vec2(12.0, 12.0),
        "Consumer",
        20.0,
        TEXT,
    );
    label(
        &painter,
        consumer.left_top() + egui::vec2(12.0, 48.0),
        &format!("Received: {}", simulation.consumer_records.len()),
        14.0,
        MUTED,
    );
    label(
        &painter,
        consumer.left_top() + egui::vec2(12.0, 74.0),
        "Consumed Records",
        13.0,
        MUTED,
    );
    for (index, record) in simulation.consumer_records.iter().enumerate() {
        let position = consumer.left_top()
            + egui::vec2(
                32.0 + (index % 2) as f32 * 48.0,
                114.0 + (index / 2) as f32 * 34.0,
            );
        token(&painter, position, &record.value, BLUE);
    }

    for message in &simulation.active_messages {
        let (start, end, color) = match (message.source, message.destination, message.path) {
            (Node::Producer, Node::Broker, MessagePath::ToBroker) => {
                (producer.right_center(), broker.left_center(), ORANGE)
            }
            (Node::Broker, Node::Broker, MessagePath::IntoTopic) => (
                broker.left_center(),
                record_position(topic, stored_count),
                ORANGE,
            ),
            (Node::Broker, Node::Consumer, MessagePath::ToConsumer) => {
                (broker.right_center(), consumer.left_center(), BLUE)
            }
            _ => continue,
        };
        let position = start + (end - start) * message.progress as f32;
        token(&painter, position, &message.value, color);
    }

    if response.clicked()
        && let Some(position) = response.interact_pointer_pos()
    {
        *selected = [
            (Node::Producer, producer),
            (Node::Broker, broker),
            (Node::Consumer, consumer),
        ]
        .into_iter()
        .find(|(_, bounds)| bounds.contains(position))
        .map(|(node, _)| node);
    }
}
