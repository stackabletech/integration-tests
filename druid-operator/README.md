# Druid Operator Tests

This package contains tests for the Apache Druid operator.

## Requirements

The tests require a running `kind` cluster with the `dev-cluster.yaml` configuration supplied in the druid-operator repository.  The tests also require a running zookeeper instance with the name `simple`.

The Druid operator itself should be running.

## Running

Once the requirements are met, a simple `cargo test` should suffice.