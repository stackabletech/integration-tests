# zookeeper-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Zookeeper%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Zookeeper%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/zookeeper-operator) for Apache ZooKeeper.

## Run tests

The integration tests are based on [KUTTL](https://kuttl.dev).

    ../create_test_cluster.py --kind kind --operator zookeeper --debug
    kubectl kuttl test




