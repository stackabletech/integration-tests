#!/bin/bash

# Bitnami Helm repo to install collaborators
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update bitnami

# Install Postgres
helm install superset-postgresql bitnami/postgresql \
    --version 11.0.0 \
    --set auth.username=superset \
    --set auth.password=superset \
    --set auth.database=superset

# Wait for Postgres to be up and running
echo Starting Postgresql database ...
while [ "$(kubectl get pod superset-postgresql-0 --output=jsonpath='{.status.containerStatuses[0].ready}')" != "true" ]; do
	sleep 2
done
echo Postgresql database started.
echo

# Execute tests
git clone -b "$GIT_BRANCH" https://github.com/stackabletech/integration-tests.git
(cd integration-tests/superset-operator && kubectl kuttl test --parallel 1)
exit_code=$?

# save logfiles and exit
./operator-logs.sh superset > /target/superset-operator.log
exit $exit_code
