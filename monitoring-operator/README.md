# monitoring-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Monitoring%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Monitoring%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/monitoring-operator) for Monitoring and Metrics. 

## Requirements

- The tests require at least 2 nodes to be available

## Content

Currently, the integration tests cover the following cases:

- **Create** a Monitoring cluster and check if it is running correctly.
- **Scale** a Monitoring cluster up (e.g., from 1 to 2 nodes) and down (e.g., from 2 to 1 nodes) and check if it is running correctly.