# nifi-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/NiFi%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/NiFi%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/nifi-operator) for Apache NiFi.

The Stackable Operator for Trino integration tests are based on [KUTTL](https://kuttl.dev). The [Secret Operator](https://github.com/stackabletech/secret-operator) has to be installed manually.

## Run Test

    ../create_test_cluster.py --kind kind --operator nifi --debug
    helm upgrade secret-operator stackable-dev/secret-operator --devel --install
    kubectl kuttl test