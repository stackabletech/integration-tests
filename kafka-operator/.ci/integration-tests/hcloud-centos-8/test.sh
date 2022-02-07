git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/kafka-operator && kubectl kuttl test)
exit_code=$?
./operator-logs.sh kafka > /target/kafka-operator.log
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
./operator-logs.sh opa > /target/opa-operator.log
./operator-logs.sh regorule > /target/regorule-operator.log
exit $exit_code
