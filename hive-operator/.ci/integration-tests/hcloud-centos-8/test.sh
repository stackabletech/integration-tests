git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/hive-operator && kubectl kuttl test)
exit_code=$?
./operator-logs.sh hive > /target/hive-operator.log
exit $exit_code
