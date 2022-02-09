# Bitnami Helm repo to install collaborators
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update bitnami

# Install Postgres
helm install airflow-postgresql bitnami/postgresql \
    --version 11.0.0 \
    --set auth.username=airflow \
    --set auth.password=airflow \
    --set auth.database=airflow

# Wait for Postgres to be up and running
echo Starting Postgresql database ...
while [ "$(kubectl get pod airflow-postgresql-0 --output=jsonpath='{.status.containerStatuses[0].ready}')" != "true" ]; do
	sleep 2
done
echo Postgresql database started.
echo

# Install Redis
helm install airflow-redis bitnami/redis \
    --set auth.password=redis

# Wait for Redis to be up and running
echo Starting Redis ...
while [ "$(kubectl get statefulset airflow-redis-master --output=jsonpath='{.status.readyReplicas}')" != "1" ] || 
      [ "$(kubectl get statefulset airflow-redis-replicas --output=jsonpath='{.status.readyReplicas}')" != "3" ]; do
    sleep 2
done
echo Redis started.
echo

# Execute tests
git clone -b $GIT_BRANCH https://github.com/stackabletech/integration-tests.git
(cd integration-tests/airflow-operator && kubectl kuttl test)
exit_code=$?

# save logfiles and exit
./operator-logs.sh airflow > /target/airflow-operator.log
exit $exit_code
