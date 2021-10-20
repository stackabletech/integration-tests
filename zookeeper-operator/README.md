# zookeeper-operator-integration-tests

[![Build Actions Status](https://github.com/stackabletech/zookeeper-operator-integration-tests/workflows/Rust/badge.svg)](https://github.com/stackabletech/zookeeper-operator-integration-tests/actions)
[![Build Actions Status](https://github.com/stackabletech/zookeeper-operator-integration-tests/workflows/Security%20audit/badge.svg)](https://github.com/stackabletech/zookeeper-operator-integration-tests/actions)
[![Build Actions Status](https://ci.stackable.tech/job/Zookeeper%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Zookeeper%20Operator%20Integration%20Tests)

This repository bundles integration tests for the [Stackable Operator](https://github.com/stackabletech/zookeeper-operator) for Apache ZooKeeper.

## Requirements

- Although the tests are written for a dynamic amount of test nodes, we recommend using the test-dev cluster with 3 nodes / agents, which was tested extensively: `./init.sh debian zookeeper-operator --scale agent=3`

## Content

Currently, the integration tests cover the following cases:

- **Create** a ZooKeeper cluster and check if it is running correctly via the [four letter commands](https://zookeeper.apache.org/doc/r3.4.14/zookeeperAdmin.html#sc_zkCommands) for version 3.5.2 and below or the [admin server commands](https://zookeeper.apache.org/doc/r3.7.0/zookeeperAdmin.html#sc_adminserver) for version 3.5.3 and above.
- **Update** a ZooKeeper cluster from version 3.4.14 to 3.5.8 and check the correctness via the four letter commands or admin server commands.
- **Scale** a ZooKeeper cluster up (e.g., from 1 to 3 nodes) and down (e.g., from 3 to 1 nodes) and check the correctness via four letter commands or admin server commands. Check the configmaps which are responsible for transmitting the zoo.cfg config properties.
- **Monitor** a ZooKeeper cluster via a prometheus endpoint. Check if JMX Explorer port is opened correctly and if required container_ports are set. 





