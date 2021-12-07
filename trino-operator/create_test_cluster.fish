#!/usr/bin/env fish

# MinIO operator chart versions from 4.2.4 to 4.3.5 (which is the latest
# at the time of writing) seem to be affected by
# https://github.com/minio/operator/issues/904
set minioOperatorChartVersion 4.2.3
set hiveOperatorVersion 0.3.0

# Create Kubernetes cluster
echo "
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
- role: worker
- role: worker
- role: worker
- role: worker
" | kind create cluster --config -

# Set up S3
helm repo add minio https://operator.min.io/
helm repo update minio
helm show values \
    --version $minioOperatorChartVersion \
    minio/minio-operator \
| sed -e "
    /requestAutoCert:/ s/:.*/: false/
    /servers:/ s/:.*/: 1/g
    /size:/ s/:.*/: 10Mi/" \
| helm install \
    --version $minioOperatorChartVersion \
    --generate-name \
    --values - \
    minio/minio-operator

echo Starting MinIO tenant ...
while test (echo (kubectl get pod \
        --selector=v1.min.io/tenant=minio1 \
        --output=jsonpath='{range .items[*]}{.status.conditions[?(@.type=="Ready")].status}{end}')) \
        != "True"
    sleep 2
end

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

set minioNodeIp \
    (kubectl get pod \
        --selector='v1.min.io/tenant=minio1' \
        --output=jsonpath="{.items[0].status.hostIP}")
set minioNodePort \
    (kubectl get service minio-external \
        --output=jsonpath="{.spec.ports[0].nodePort}")
 
set -Ux S3_ENDPOINT "http://$minioNodeIp:$minioNodePort"
set -Ux S3_ACCESS_KEY \
    (kubectl get secret minio1-secret \
        --output=jsonpath="{.data.accesskey}" | base64 --decode)
set -Ux S3_SECRET_KEY \
    (kubectl get secret minio1-secret \
        --output=jsonpath="{.data.secretkey}" | base64 --decode)

# Deploy Hive and Trino operators
helm repo add stackable-dev https://repo.stackable.tech/repository/helm-dev
helm repo add stackable-stable https://repo.stackable.tech/repository/helm-stable
helm repo update
helm install hive-operator stackable-stable/hive-operator --version=$hiveOperatorVersion
helm install trino-operator stackable-dev/trino-operator --devel
