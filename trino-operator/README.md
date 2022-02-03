# trino-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Trino%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Trino%20Operator%20Integration%20Tests)

The Stackable Operator for Trino integration tests are based on [KUTTL](https://kuttl.dev). The [Secret Operator](https://github.com/stackabletech/secret-operator) has to be installed manually.

    ../create_test_cluster.py --kind kind --operator trino --debug
    helm upgrade secret-operator stackable-dev/secret-operator --devel --install
    kubectl kuttl test

In case of failure: The download of Trino (~3gb) might cause a timeout (720s). It should work the second time if your internet connection is slow, or download the image in advance.
