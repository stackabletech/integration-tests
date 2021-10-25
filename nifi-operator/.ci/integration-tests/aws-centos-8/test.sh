/stackable.sh testdriver-1 -i /.cluster/key 'sudo sh -c "echo \"13.32.25.75     static.rust-lang.org\" >> /etc/hosts"'
/stackable.sh testdriver-1 -i /.cluster/key 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
/stackable.sh testdriver-1 -i /.cluster/key 'cargo --version'
/stackable.sh testdriver-1 -i /.cluster/key 'sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git -y'
/stackable.sh testdriver-1 -i /.cluster/key "git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git"
/stackable.sh testdriver-1 -i /.cluster/key 'cd integration-tests/ && cargo test --package nifi-operator-integration-tests -- --test-threads=1'
exit_code=$?
/stackable.sh main-1 -i /.cluster/key 'journalctl -u stackable-agent' > /target/main-1-stackable-agent.log
/stackable.sh main-2 -i /.cluster/key 'journalctl -u stackable-agent' > /target/main-2-stackable-agent.log
/stackable.sh main-3 -i /.cluster/key 'journalctl -u stackable-agent' > /target/main-3-stackable-agent.log
/stackable.sh orchestrator -i /.cluster/key 'journalctl -u stackable-nifi-operator' > /target/stackable-nifi-operator.log
/stackable.sh orchestrator -i /.cluster/key 'journalctl -u stackable-zookeeper-operator' > /target/stackable-zookeeper-operator.log
exit $exit_code
