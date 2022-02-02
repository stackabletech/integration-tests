kubectl label nodes main-1.stackable.test node=1
kubectl label nodes main-1.stackable.test nodeType=druid-data
kubectl label nodes main-2.stackable.test node=2
kubectl label nodes main-3.stackable.test node=3
ssh testdriver-1 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
ssh testdriver-1 'cargo --version'
ssh testdriver-1 'sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git --nobest -y'
ssh testdriver-1 "git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git"
ssh testdriver-1 'cd integration-tests/druid-operator && cargo test -- --nocapture --test-threads=1'
exit_code=$?
./operator-logs.sh druid > /target/druid-operator.log
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
exit $exit_code
