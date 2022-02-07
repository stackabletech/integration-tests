# airflow-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Airflow%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Airflow%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/airflow-operator) for Apache Airflow.

## Requirements

A running cluster is required - e.g. a kind cluster can be set up like this:

    kind create cluster

## Running

The integration tests are based on [KUTTL](https://kuttl.dev) and be run with this command:

    kubectl kuttl test