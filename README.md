# Stackable Operator Integration Tests

This repository contains integration tests for Stackable operators.
We decided to keep them in a separate repository.

There is a helper script called `create_test_cluster.py` which can be used to install a multi-node `kind` cluster, 

## Requirements

You need working versions of the following tools in your PATH:
- kubectl
- kind
- helm
- python (version 3, tested with 3.9) 
- pip (version 3) for hive tests

### WARN: Helm bug

Please be aware that Helm currently (as of version 3.7.2) has a [bug](https://github.com/helm/helm/pull/10519), which might mean the script can abort with an error:

    Error: no repositories to show

If this happens please add a (any) repository to Helm manually (this can be deleted again):

    helm repo add stackable-dev https://repo.stackable.tech/repository/helm-dev

To be more precise: Helm needs to find a valid `repositories.yaml` file in its config directory. 

## Set up a test kind-cluster

The `create_test_cluster.py` utility script will set up a test kind cluster (if requested with the `--kind` parameter) and install dependencies required for running the integration tests.

    . create_test_cluster.py -o <operator> -v [version] --kind

Example

    . create_test_cluster.py -o trino --kind

This will set up a three node kind cluster called `integration-tests` and install the `trino-operator` along with a MiniIO cluster and the `hive-operator`. When this is done, you can run the integration tests for the `trino-operator` by following the instructions below.

IMPORTANT: The script might ask you to set environment variables that are needed for the integration tests!

## Run tests

It is recommended to run the tests in the same shell the was used to create the Kind cluster. This is to ensure that any required environment variables are available to the test process.

    cargo test --package zookeeper-operator-integration-tests -- --nocapture --test-threads=1

## Build commons

    cargo build --package integration-test-commons

## Tips and tricks

### Test images locally

If you want to test product or operator images locally before publishing them to the image registry, you can build them locally and then load them in your `kind` cluster like this:

   kind load docker-image docker.stackable.tech/stackable/superset:1.3.2-stackable0  --name integration-tests --verbosity 999


