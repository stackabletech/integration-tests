(cd /test-suite && kubectl kuttl test)
exit_code=$?
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
exit $exit_code
