## Set-up a test kind-cluster

The `create_test_cluster.sh` utility script will set up a test kind cluster and install dependencies required for running the integration tests.

    . create_test_cluster.sh <operator> [version]

Example

    . create_test_cluster.sh trino

This will set up a three node kind cluster called `integration-tests` and install the `trino-operator` along with a MiniIO cluster and the `hive-operator`. When this is done, you can run the integration tests for the `trino-operator` by following the instructions below.

IMPORTANT: Use the dot notation (or `source`) to run the `create_test_cluster.sh` to make sure that any environment variables created are available to the integration tests.

## Run tests

It is recommended to run the tests in the same shell the was used to create the Kind cluster. This is to ensure that any required environment variables are available to the test process.

    cargo test --package zookeeper-operator-integration-tests -- --nocapture --test-threads=1

## Build commons

    cargo build --package integration-test-commons

## Tips and tricks

### Test images locally

If you want to test product or operator images locally before publishing them to the image registry, you can build them locally and then load them in your `kind` cluster like this:

   kind load docker-image docker.stackable.tech/stackable/superset:1.3.2-stackable0  --name integration-tests --verbosity 999


