# kafka-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Kafka%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Kafka%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/kafka-operator) for Apache Kafka. 

## Requirements

- The tests require at least 3 nodes to be available

## Content

Currently, the integration tests cover the following cases:

- **Create** a Kafka cluster and check if it is running correctly.
- **Scale** a Kafka cluster up (e.g., from 1 to 3 nodes) and down (e.g., from 3 to 1 nodes) and check if it is running correctly.
- **Monitor** a Kafka cluster via a prometheus endpoint. Check if JMX Explorer port is opened correctly and if required container_ports are set.