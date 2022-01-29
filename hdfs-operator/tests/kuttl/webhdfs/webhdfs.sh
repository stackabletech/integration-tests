#!/usr/bin/env bash

set -eo pipefail

COMMAND=$1

NAMENODE_NAME="hdfs-namenode-default-0"

#BASE_URL=$(hdfs_base_url)
BASE_URL=http://${NAMENODE_NAME}:9870

case ${COMMAND} in
 ls)
    curl  -i -L "${BASE_URL}/webhdfs/v1/testdata.txt?user.name=stackable&op=LISTSTATUS"
 ;;
 create)
    curl  -i -L -XPUT -T $(dirname $0)/testdata.txt "${BASE_URL}/webhdfs/v1/testdata.txt?op=CREATE&user.name=stackable"
 ;;
esac
