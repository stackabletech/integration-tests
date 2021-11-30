## Set-up a test kind-cluster

The `create_test_cluster.sh` utility script will set up a test kind cluster and install dependencies required for running the integration tests.

  . create\_test\_cluster.sh <operator> [version]

Example

  . create\_test\_cluster.sh superset

This will set up a three node kind cluster called `integration-tests` and install the `superset-operator` along with a MiniIO cluster, the `trino-operator` and the `hive-operator`. When this is done, you can run the integration tests for the `superset-operator` by following the instructions below.

IMPORTANT: Use the dot notation (or `source`) to run the `create_test_cluster.sh` to make sure that any environment variables created are available to the integration tests.

## Run tests

It is recommended to run the tests in the same shell the was used to create the Kind cluster. This is to ensure that any required environment variables are available to the test process.

    cargo test --package zookeeper-operator-integration-tests -- --nocapture --test-threads=1

## Build commons

    cargo build --package integration-tests-commons

