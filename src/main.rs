mod model;
mod visual;

use model::{Broker, Consumer, Producer};

fn main() -> eframe::Result<()> {
    let mode = std::env::args().nth(1);
    match mode.as_deref() {
        Some("model") => {
            run_model_demo();
            Ok(())
        }
        Some("visual") => visual::run(),
        _ => {
            eprintln!("Usage: cargo run -- <model|visual>");
            std::process::exit(2);
        }
    }
}

fn run_model_demo() {
    let mut broker = Broker::new();
    broker
        .create_topic("temperature")
        .expect("demo topic should be created");
    println!("[Broker] created topic: temperature\n");

    let producer = Producer;
    for value in ["30", "31", "29", "32"] {
        println!("[Producer] publish topic=temperature value={value}");
        producer
            .publish(&mut broker, "temperature", value)
            .expect("demo publish should succeed");
        println!("[Broker] stored topic=temperature value={value}\n");
    }

    let consumer = Consumer;
    println!("[Consumer] reading topic=temperature");
    let records = consumer
        .consume(&broker, "temperature")
        .expect("demo topic should be readable");
    for record in records {
        println!("{}", record.value);
    }
}
