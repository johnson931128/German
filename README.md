# German

A small distributed commit-log system written in Rust for learning distributed systems.

## Overview

German is an educational project inspired by Apache Kafka's architecture. It is not a Kafka clone and does not target Kafka protocol compatibility or production readiness.

## Current Scope

The repository currently includes a small in-memory Producer → Broker → Topic → Consumer model. Run `cargo run -- model` to publish four sample temperature records and read them back. This is an educational model with no networking, persistence, partitioning, or replication; it does not implement Kafka protocol compatibility.

Run `cargo run -- visual` to open an event-driven simulation canvas of the same Producer → Broker → Topic → Consumer data flow. Animated Record tokens travel between nodes; the temperature Topic stays inside the Broker and displays actual model state. This remains in-memory only, with no networking, persistence, partition, replication, or offset tracking.

## Long-term Direction

Build understanding incrementally, starting with a single broker and later exploring multiple brokers, replication, failure handling, coordination, and related distributed systems concepts.

## Build

```powershell
cargo build
```

## Run

```powershell
cargo run -- model
```

Start the educational visualization:

```powershell
cargo run -- visual
```

The fixed scenario generates `30`, `31`, `29`, and `32` at two-second simulated intervals. Each Record is sent to the Broker, received, then stored in `temperature`. At simulated time 8.0s the Consumer requests all stored Records, which are delivered with a second set of token animations.

Use **Play**, **Pause**, **Step**, and **Reset** below the canvas. Step pauses playback and executes exactly the next event. Speed can be set to 0.5x, 1x, or 2x. Click a node for a brief state readout; the event log uses simulated timestamps. Playback and stepping use the same deterministic event queue, and the simulation layer calls the existing model rather than implementing another Broker.

## Test

```powershell
cargo test
```

## Project Status

The project remains at the foundation stage. The model demonstrates basic publish/subscribe responsibilities, while Partition, append-only log, and offset behavior from Phase 1 remain unimplemented.
