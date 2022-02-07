# hbase-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/HBase%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/HBase%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/hbase-operator) for Apache HBase.

## Run tests

The integration tests are based on [KUTTL](https://kuttl.dev).

    ../create_test_cluster.py --kind --operator hbase
    kubectl kuttl test
