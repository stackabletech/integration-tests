git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/hbase-operator && kubectl kuttl test)
exit_code=$?
./operator-logs.sh hbase > /target/hbase-operator.log
./operator-logs.sh hdfs > /target/hdfs-operator.log
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
exit $exit_code
