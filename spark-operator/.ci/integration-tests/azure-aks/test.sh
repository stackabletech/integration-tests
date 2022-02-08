git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
sleep 1200
(cd integration-tests/spark-operator && kubectl kuttl test)
exit_code=$?
./operator-logs.sh spark > /target/spark-operator.log
exit $exit_code
