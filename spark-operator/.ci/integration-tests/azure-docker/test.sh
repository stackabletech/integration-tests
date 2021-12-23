docker run --rm --volume $(echo $KUBECONFIG_FOLDER_VOLUME):/root/.kube \
    docker.stackable.tech/integration-tests:latest \
    spark-operator-integration-tests -- --nocapture --test-threads=1
exit_code=$?
./operator-logs.sh spark > /target/spark-operator.log
exit $exit_code
