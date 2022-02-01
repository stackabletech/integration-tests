# install stuff on testdriver
ssh testdriver-1 sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git -y --nobest

# install Rust with a script on testdriver
echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y" > install-rust.sh
scp install-rust.sh testdriver-1:install-rust.sh
ssh testdriver-1 chmod a+x install-rust.sh
ssh testdriver-1 ./install-rust.sh

# clone integration test repository on testdriver
ssh testdriver-1 git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git

# create/execute script which installs Python and required PIP modules
# (we build it ourselves as the official version in CentOS8 is not sufficient for one of the modules)
echo "yum groupinstall 'development tools' -y && yum install wget openssl-devel bzip2-devel libffi-devel xz-devel -y
cd /opt
wget https://www.python.org/ftp/python/3.9.6/Python-3.9.6.tgz
tar xzf Python-3.9.6.tgz
rm Python-3.9.6.tgz
cd Python-3.9.6
./configure --enable-optimizations
make install
cd
pip3 install -r integration-tests/hive-operator/python/requirements.txt
" > install-python.sh
scp install-python.sh testdriver-1:install-python.sh
ssh testdriver-1 chmod a+x install-python.sh
ssh testdriver-1 ./install-python.sh

# create script which runs the test and execute it
echo "cd integration-tests/hive-operator
cargo test -- --nocapture
" > run-test.sh
scp run-test.sh testdriver-1:run-test.sh
ssh testdriver-1 chmod a+x run-test.sh
ssh testdriver-1 ./run-test.sh
exit_code=$?
./operator-logs.sh hive > /target/hive-operator.log
exit $exit_code
