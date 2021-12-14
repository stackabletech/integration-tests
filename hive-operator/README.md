# hive-operator-integration-tests

[![Build Actions Status](https://ci.stackable.tech/job/Hive%20Operator%20Integration%20Tests/badge/icon?subject=Integration%20Tests)](https://ci.stackable.tech/job/Hive%20Operator%20Integration%20Tests)

In order to test hive we need the `hive-metastore-client` and `thrift` python library.

## Installation

The following steps for apt:

`sudo apt-get update`

### Python

`sudo apt-get install python3`

### Python pip

`sudo apt-get install python3-pip`

### Dependencies

Assuming current directory ./hive-operator:

`pip3 install -r python/requirements.txt`
