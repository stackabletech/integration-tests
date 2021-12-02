ssh testdriver-1 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
ssh testdriver-1 'cargo --version'
ssh testdriver-1 'sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git -y'
ssh testdriver-1 "git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git"
ssh testdriver-1 'cd integration-tests/kafka-operator && cargo test -- --nocapture'
exit_code=$?
./operator-logs.sh kafka > /target/kafka-operator.log
./operator-logs.sh zookeeper > /target/zookeeper-operator.log
exit $exit_code
