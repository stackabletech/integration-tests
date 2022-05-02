# superset-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Superset%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Superset%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/superset-operator) for Apache Superset.

## Run tests

The integration tests are based on [KUTTL](https://kuttl.dev).

    ./create_test_cluster.py --kind kind --operator superset --debug
    cd superset-operator
    kubectl kuttl test

## Test Description

- **smoke**: Tests whether a cluster can be installed and a login is possible
- **druid-connection**: Tests whether a druid connection is set up correctly if the druid instance is started *after* the DruidConnection has been deployed.