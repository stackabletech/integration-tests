# spark-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Spark%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Spark%20Operator%20Integration%20Tests)

Integration tests for the Stackable Operator for Apache Spark.

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/spark-operator) for Apache Spark. 

## Requirements

- The requires at least 3 available nodes


## Content

Currently, the integration tests cover the following cases:

- **Create** a Spark cluster and check if it is running correctly
- **Update** a Spark cluster from version 3.0.1 to 3.1.1 and check if it is running correctly
- **Scale** a Spark cluster up (e.g. from 1 to 2 workers) and down (e.g. from 2 to 1 workers)
- **Restart** a Spark cluster via Restart command
