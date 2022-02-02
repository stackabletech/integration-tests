# Write script to set up Postgres and execute it on testdriver-1
echo "helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update bitnami
helm install airflow-postgresql bitnami/postgresql \
    --set postgresqlUsername=airflow \
    --set postgresqlPassword=airflow \
    --set postgresqlDatabase=airflow
" > set-up-postgres.sh
scp set-up-postgres.sh testdriver-1:set-up-postgres.sh
ssh testdriver-1 chmod a+x set-up-postgres.sh
ssh testdriver-1 ./set-up-postgres.sh

# Wait for Postgres to be up and running
echo Starting Postgresql database ...
while [ "$(kubectl get pod airflow-postgresql-postgresql-0 --output=jsonpath='{.status.containerStatuses[0].ready}')" != "true" ]; do
	sleep 2
done
echo Postgresql database started.
echo


# Write script to set up Redis and execute it on testdriver-1
echo "helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update bitnami
helm install airflow-redis bitnami/redis \
    --set auth.password=redis
" > set-up-redis.sh
scp set-up-redis.sh testdriver-1:set-up-redis.sh
ssh testdriver-1 chmod a+x set-up-redis.sh
ssh testdriver-1 ./set-up-redis.sh

# Wait for Redis to be up and running
echo Starting Redis ...
while [ "$(kubectl get statefulset airflow-redis-master --output=jsonpath='{.status.readyReplicas}')" != "1" ] || 
      [ "$(kubectl get statefulset airflow-redis-replicas --output=jsonpath='{.status.readyReplicas}')" != "3" ]; do
    sleep 2
done
echo Redis started.
echo

# install stuff on testdriver
ssh testdriver-1 sudo yum install vim procps curl gcc make pkgconfig openssl-devel systemd-devel python3-pip container-selinux selinux-policy-base git --nobest -y

# install Rust with a script on testdriver
echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y" > install-rust.sh
scp install-rust.sh testdriver-1:install-rust.sh
ssh testdriver-1 chmod a+x install-rust.sh
ssh testdriver-1 ./install-rust.sh

# clone integration test repository on testdriver
ssh testdriver-1 git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git

# create script which runs the test and execute it
echo "cd integration-tests/airflow-operator
cargo test -- --nocapture
" > run-test.sh
scp run-test.sh testdriver-1:run-test.sh
ssh testdriver-1 chmod a+x run-test.sh
ssh testdriver-1 ./run-test.sh
exit_code=$?
./operator-logs.sh airflow > /target/airflow-operator.log
exit $exit_code
