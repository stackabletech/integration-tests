#!/bin/env bash

set -eo pipefail

COMMAND=$1

NAMENODE_NAME="hdfs-namenode-default-0"

hdfs_base_url() {
    local NODE_NAME=$(kubectl get pods -o jsonpath="{.items[?(@.metadata.name==\"$NAMENODE_NAME\")].spec.nodeName}")
    local NODE_IP=$(kubectl get nodes -o jsonpath="{.items[?(@.metadata.name==\"$NODE_NAME\")].status.addresses[?(@.type=='InternalIP')].address}")
    local NODE_HTTP_PORT=$(kubectl get svc -o jsonpath="{.items[?(@.metadata.name==\"$NAMENODE_NAME\")].spec.ports[?(@.name=='http')].nodePort}")

    echo "http://${NODE_IP}:${NODE_HTTP_PORT}"
}

BASE_URL=$(hdfs_base_url)

case ${COMMAND} in
 ls)
    curl  -i "${BASE_URL}/webhdfs/v1/testdata.txt?user.name=stackable&op=LISTSTATUS"
 ;;
 create)
    curl  -i -XPUT -T $(dirname $0)/testdata.txt "${BASE_URL}/webhdfs/v1/testdata.txt?op=CREATE&user.name=stackable&noredirect=true"
 ;;
esac
