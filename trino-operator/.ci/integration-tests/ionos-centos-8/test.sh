export MINIO_OPERATOR_CHART_VERSION="4.2.3"

# Write script to set up S3 storage and execute it on testdriver-1
echo "helm repo add minio https://operator.min.io/
helm repo update minio
helm show values \\
    --version $MINIO_OPERATOR_CHART_VERSION \\
    minio/minio-operator \\
| sed -e \" 
    /requestAutoCert:/ s/:.*/: false/ 
    /servers:/ s/:.*/: 1/g 
    /size:/ s/:.*/: 10Mi/ 
    /storageClassName:/ s/:.*/: local-path/\" \\
| helm install \
    --version $MINIO_OPERATOR_CHART_VERSION \
    --generate-name \
    --values - \
    minio/minio-operator
" > set-up-s3.sh
scp set-up-s3.sh testdriver-1:set-up-s3.sh
ssh testdriver-1 chmod a+x set-up-s3.sh
ssh testdriver-1 ./set-up-s3.sh

# Wait for MinIO tenant to be up and running
echo Starting MinIO tenant ...
while [ "$(kubectl get pod  --selector=v1.min.io/tenant=minio1 --output=jsonpath='{range .items[*]}{.status.conditions[?(@.type=="Ready")].status}{end}')" != "True" ]; do
	sleep 2
done
echo MinIO tenant started.
echo

# Create Service w/ NodePort
echo "
apiVersion: v1
kind: Service
metadata:
  name: minio-external
spec:
  type: NodePort
  selector:
    v1.min.io/tenant: minio1
  ports:
    - port: 80
      targetPort: 9000
" | kubectl apply -f -

sleep 30

# Collect data for tests
export MINIO_NODE_IP=$(kubectl get pod --selector='v1.min.io/tenant=minio1' --output=jsonpath='{.items[0].status.hostIP}')
export MINIO_NODE_PORT=$(kubectl get service minio-external --output=jsonpath='{.spec.ports[0].nodePort}')
export S3_ENDPOINT="http://$MINIO_NODE_IP:$MINIO_NODE_PORT"
export S3_ACCESS_KEY=$(kubectl get secret minio1-secret --output=jsonpath="{.data.accesskey}" | base64 --decode)
export S3_SECRET_KEY=$(kubectl get secret minio1-secret --output=jsonpath="{.data.secretkey}" | base64 --decode)

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
echo "export S3_ENDPOINT=$S3_ENDPOINT
export S3_ACCESS_KEY=$S3_ACCESS_KEY
export S3_SECRET_KEY=$S3_SECRET_KEY
cd integration-tests/trino-operator
cargo test -- --nocapture
" > run-test.sh
scp run-test.sh testdriver-1:run-test.sh
ssh testdriver-1 chmod a+x run-test.sh
ssh testdriver-1 ./run-test.sh
exit_code=$?
./operator-logs.sh trino > /target/trino-operator.log
./operator-logs.sh hive > /target/hive-operator.log
exit $exit_code
