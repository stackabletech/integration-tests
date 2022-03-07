# Stackable Operator Integration Tests

This repository contains integration tests for Stackable operators.
We decided to keep them in a separate repository.

There is a helper script called `create_test_cluster.py` which can be used to install a multi-node `kind` cluster, 

## Requirements

You need working versions of the following tools in your PATH:
- [kubectl](https://kubernetes.io/docs/tasks/tools/#kubectl)
- [kind](https://kind.sigs.k8s.io/), optional - only if the `--kind` option is used (tested with 0.11)
- [helm](https://helm.sh/) (tested with 3.7.2)
- [python](https://www.python.org/) (version 3.7 or above) 
- [pip](https://pip.pypa.io/en/stable/) (version 3.x), optional for Hive tests

### WARN: Helm bug

Please be aware that Helm currently (as of version 3.7.2) has a [bug](https://github.com/helm/helm/pull/10519), which might mean the script can abort with an error:

    Error: no repositories to show

If this happens please add a (any) repository to Helm manually (this can be deleted again):

    helm repo add stackable-dev https://repo.stackable.tech/repository/helm-dev

To be more precise: Helm needs to find a valid `repositories.yaml` file in its config directory. 

    ./create_test_cluster.py --debug --kind --operator <operator>

## Set up a test kind-cluster

The `create_test_cluster.py` utility script will set up a test kind cluster (if requested with the `--kind` parameter) and install dependencies required for running the integration tests.

    ./create_test_cluster.py --debug --kind --operator <operator>

Example

    ./create_test_cluster.py --debug --kind --operator trino

This will set up a three node kind cluster called `integration-tests` and install the `trino-operator` along with a MiniIO cluster and the `hive-operator`. When this is done, you can run the integration tests for the `trino-operator` by following the instructions below.
It is possible to specify multiple operators to install at the same time. Operator versions can be specified in the format 'operator[=version]'.

Example

    ./create_test_cluster.py --kind --operator trino superset=0.3.0

IMPORTANT: The script might ask you to set environment variables that are needed for the integration tests!

## Set up a test kind-cluster with monitoring

This will install the [Prometheus Operator](https://prometheus-operator.dev) and a general ServiceMonitor to scrape metrics as described [here](https://docs.stackable.tech/home/monitoring.html). 

Example

    ./create_test_cluster.py --debug --kind --prometheus --operator trino

## Set up a test kind-cluster with cluster examples deployed

This will start the Trino operator and deploy the `simple-<operator>-cluster.yaml` example in the operator's `examples` folder:

    ./create_test_cluster.py --debug --kind --operator trino --example

## Access deployed services

There is a helper script that automatically forwards the deployed services to your local machine so that they are accessible.
You can run it as follows, press Ctrl + C to exit it again:

    ./access_services.py

## Run KUTTL tests

Currently, some integration tests are adapted to utilize [KUTTL](https://kuttl.dev) instead if Rust. Install KUTTL first as described [here](https://kuttl.dev/docs/cli.html#setup-the-kuttl-kubectl-plugin).
You can run KUTTL tests (if there is a folder named `tests/kuttl`) via:

    cd <operator>
    kubectl kuttl test

## Tips and tricks

### Test images locally

If you want to test product or operator images locally before publishing them to the image registry, you can build them locally and then load them in your `kind` cluster like this:

    kind load docker-image docker.stackable.tech/stackable/trino:362-stackable0  --name integration-tests --verbosity 999


