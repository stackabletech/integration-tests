## Requirements

You need working versions of the following tools in your PATH:
- kubectl
- kind
- helm
- python (version 3)
- pip (version 3)

## Set-up a test kind-cluster

The `create_test_cluster.py` utility script will set up a test kind cluster and install dependencies required for running the integration tests.

    ./create_test_cluster.py --debug --kind --operator <operator>

Example

    ./create_test_cluster.py --debug --kind --operator trino

This will set up a three node kind cluster called `integration-tests` and install the `trino-operator` along with a MiniIO cluster and the `hive-operator`. When this is done, you can run the integration tests for the `trino-operator` by following the instructions below.

## Run tests

It is recommended to run the tests in the same shell the was used to create the Kind cluster. This is to ensure that any required environment variables are available to the test process.

    cargo test --package trino-operator-integration-tests -- --nocapture --test-threads=1

## Build commons

    cargo build --package integration-test-commons

## Tips and tricks

### Test images locally

If you want to test product or operator images locally before publishing them to the image registry, you can build them locally and then load them in your `kind` cluster like this:

   kind load docker-image docker.stackable.tech/stackable/trino:362-stackable0  --name integration-tests --verbosity 999


