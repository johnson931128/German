# German

A small distributed commit-log system written in Rust for learning distributed systems.

## Overview

German is an educational project inspired by Apache Kafka's architecture. It is not a Kafka clone and does not target Kafka protocol compatibility or production readiness.

## Current Scope

The repository currently includes a small in-memory Producer → Broker → Topic → Consumer model. Run `cargo run -- model` to publish four sample temperature records and read them back. This is an educational model with no networking, persistence, partitioning, or replication; it does not implement Kafka protocol compatibility.

## Long-term Direction

Build understanding incrementally, starting with a single broker and later exploring multiple brokers, replication, failure handling, coordination, and related distributed systems concepts.

## Build

```powershell
cargo build
```

## Run

```powershell
cargo run
```

## Test

```powershell
cargo test
```

## Project Status

The project remains at the foundation stage. The model demonstrates basic publish/subscribe responsibilities, while Partition, append-only log, and offset behavior from Phase 1 remain unimplemented.
