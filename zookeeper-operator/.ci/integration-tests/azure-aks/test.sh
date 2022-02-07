kubectl get nodes > /target/k8s_nodes.txt
git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/zookeeper-operator && kubectl kuttl test)
exit_code=$?
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
exit $exit_code
