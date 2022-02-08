git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/druid-operator && kubectl kuttl test)
exit_code=$?
./operator-logs.sh druid > /target/druid-operator.log
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
exit $exit_code
