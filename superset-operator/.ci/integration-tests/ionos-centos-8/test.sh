# Write script to set up Postgres and execute it on testdriver-1
echo "helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update bitnami
helm install superset bitnami/postgresql \
    --set postgresqlUsername=superset \
    --set postgresqlPassword=superset \
    --set postgresqlDatabase=superset
" > set-up-postgres.sh
scp set-up-postgres.sh testdriver-1:set-up-postgres.sh
ssh testdriver-1 chmod a+x set-up-postgres.sh
ssh testdriver-1 ./set-up-postgres.sh

# Wait for Postgres to be up and running
echo Starting Postgresql database ...
while [ "$(kubectl get pod superset-postgresql-0 --output=jsonpath='{.status.containerStatuses[0].ready}')" != "true" ]; do
	sleep 2
done
echo Postgresql database started.
echo

# install stuff on testdriver
ssh testdriver-1 sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git -y --nobest

# install Rust with a script on testdriver
echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y" > install-rust.sh
scp install-rust.sh testdriver-1:install-rust.sh
ssh testdriver-1 chmod a+x install-rust.sh
ssh testdriver-1 ./install-rust.sh

# clone integration test repository on testdriver
ssh testdriver-1 git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git

# create script which runs the test and execute it
echo "cd integration-tests/superset-operator
cargo test -- --nocapture
" > run-test.sh
scp run-test.sh testdriver-1:run-test.sh
ssh testdriver-1 chmod a+x run-test.sh
ssh testdriver-1 ./run-test.sh
exit_code=$?
./operator-logs.sh superset > /target/superset-operator.log
exit $exit_code
