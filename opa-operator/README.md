# opa-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/OPA%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/OPA%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/opa-operator) for the OpenPolicyAgent (OPA).

## Requirements

- The tests require at least 2 nodes to be available

## Usage

Please refer to the [test-dev-cluster](https://github.com/stackabletech/test-dev-cluster) instructions on how to set up and run the integration tests.

## Content

Currently, the integration tests cover the following cases:

- **Create** an OPA cluster and check if it is running correctly.
- **Scale** an OPA cluster up (e.g., from 1 to 2 nodes) and down (e.g., from 2 to 1 nodes) and check if it is running correctly.
- **Monitor** an OPA cluster via a prometheus endpoint. Check that monitoring / metrics port is open and required container_ports are set.