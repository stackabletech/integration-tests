# nifi-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/NiFi%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/NiFi%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/nifi-operator) for Apache NiFi.

## Requirements

- The tests require at least 2 nodes to be available

## Content

Currently, the integration tests cover the following cases:

- **Create** a NiFi cluster and check if it is running correctly.
- **Scale** a NiFi cluster up (e.g., from 1 to 2 nodes) and down (e.g., from 2 to 1 nodes) and check if it is running correctly.
- **Monitor** a NiFi cluster. Check monitoring port is opened correctly and if required container_ports are set.

## Important
There is a timeout in the integration-test-commons (kube-rs) library that requires a pod to be ready in under 295 seconds. This is a Kubernetes watch timeout. For slow internet connection (package download) or slow cpu (unpacking) this timeout maybe too low and cause tests to panic.