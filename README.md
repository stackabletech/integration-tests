## Set-up a test kind-cluster

  ./create_test_cluster.sh <operator> [version]

Example

  ./create_test_cluster.sh superset

This will set up a three node kind cluster called `integration-tests` and install the `superset-operator` along with a MiniIO cluster, the `trino-operator` and the `hive-operator`. When this is done, you can run the integration tests for the `superset-operator` by following the instructions below.

## Run tests

    cargo test --package zookeeper-operator-integration-tests

## Build commons

    cargo build --package integration-tests-commons

