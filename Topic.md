# German

German is a small distributed commit-log system written in Rust for learning distributed systems.
Its architecture is inspired by Apache Kafka, but the goal is to learn distributed systems concepts, not to clone Kafka or provide Kafka protocol compatibility.
A Producer will send Records to a Broker.
The Broker will append Records to an ordered log associated with a Topic and Partition.
A Consumer will read Records by offset.
The first implementation steps will establish a runnable project and a small single-broker log model.
Later steps may add multiple Brokers, replication, leader/follower roles, failure handling, and leader election.
The project may explore consistency, consensus, checkpoints, and stream processing as learning progresses.
Features will be added incrementally without designing a complete system in advance.
The system will use a CLI, log output, and observable runtime state to make behavior understandable.
This is an educational project, not a complete Kafka reimplementation or a production-ready system.
