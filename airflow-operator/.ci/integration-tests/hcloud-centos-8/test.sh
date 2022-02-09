# Execute tests
git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/airflow-operator && kubectl kuttl test)
exit_code=$?

# save logfiles and exit
./operator-logs.sh airflow > /target/airflow-operator.log
exit $exit_code
