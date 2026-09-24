# German

A small distributed commit-log system written in Rust for learning distributed systems.

## Overview

German is an educational project inspired by Apache Kafka's architecture. It is not a Kafka clone and does not target Kafka protocol compatibility or production readiness.

## Current Scope

The repository currently contains a minimal Rust CLI baseline. Broker, Topic, Partition, Record, log, producer, consumer, and offset behavior are planned and are not implemented yet.

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

The project is at the foundation stage. The Rust CLI builds, but distributed log functionality is not implemented yet.
