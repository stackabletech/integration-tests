# hdfs-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/HDFS%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/HDFS%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/hdfs-operator) for Apache HDFS. 

## Run tests

The integration tests are based on [KUTTL](https://kuttl.dev).

    ./create_test_cluster.py --kind kind --operator hdfs --debug
    cd hdfs-operator
    kubectl kuttl test