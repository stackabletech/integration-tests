ssh testdriver-1 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
ssh testdriver-1 'cargo --version'
ssh testdriver-1 'sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git -y'
ssh testdriver-1 "git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git"
ssh testdriver-1 'cd integration-tests/opa-operator && cargo test -- --nocapture --test-threads=1'
exit_code=$?
./operator-logs.sh opa > /target/opa-operator.log
./operator-logs.sh regorule > /target/regorule-operator.log
exit $exit_code
