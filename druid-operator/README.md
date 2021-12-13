# Druid Operator Tests

[![Build Actions Status](https://ci.stackable.tech/job/Druid%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Druid%20Operator%20Integration%20Tests)

This package contains tests for the Apache Druid operator.

## Requirements

The tests require a running `kind` cluster with the `dev-cluster.yaml` configuration supplied in the druid-operator repository.  The tests also require a running zookeeper instance with the name `simple`.

The Druid operator itself should be running.

## Running

Once the requirements are met, a simple `cargo test` should suffice.