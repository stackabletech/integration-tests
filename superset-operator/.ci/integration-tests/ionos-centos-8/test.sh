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
