# opa-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/OPA%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/OPA%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/opa-operator) for the OpenPolicyAgent (OPA).

## Requirements

OPA is deployed as Daemonset and the integration tests expect to 3 nodes (and 3 OPA pods) to be deployed.
Any other amount of nodes will cause the tests to fail.

## Run tests

The integration tests are based on [KUTTL](https://kuttl.dev). 

    ../create_test_cluster.py --kind kind --operator opa --debug
    kubectl kuttl test
