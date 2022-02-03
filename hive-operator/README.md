# hive-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Hive%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Hive%20Operator%20Integration%20Tests)

## Run tests

The integration tests are based on [KUTTL](https://kuttl.dev).

    ./create_test_cluster.py --kind kind --operator hive --debug
    cd hive-operator
    kubectl kuttl test
