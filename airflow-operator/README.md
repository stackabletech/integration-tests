# airflow-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Airflow%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Airflow%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/airflow-operator) for Apache Airflow.

## Requirements

The simplest way to set-up a cluster and the necessary dependencies is to use the 
`create_test_cluster.py` script in the root directory:

    ./create_test_cluster.py --debug --kind --operator airflow

## Running

Once the requirements are met, a simple `cargo test` can be issued to run the tests.