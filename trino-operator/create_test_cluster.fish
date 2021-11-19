#!/usr/bin/env fish

# Create Kubernetes cluster
kind create cluster

# Set up S3
helm repo add minio https://operator.min.io/
helm repo update minio
helm install \
    --namespace minio-operator \
    --create-namespace \
    --generate-name \
    --set tenants[0].name=minio1 \
    --set tenants[0].namespace=default \
    --set tenants[0].pools[0].servers=1 \
    --set tenants[0].pools[0].size=10Mi \
    --set tenants[0].pools[0].storageClassName=standard \
    --set tenants[0].secrets.enabled=true \
    --set tenants[0].secrets.name=minio1-secret \
    --set tenants[0].secrets.accessKey=minio \
    --set tenants[0].secrets.secretKey=minio123 \
    --set tenants[0].certificate.requestAutoCert=false \
    minio/minio-operator

echo Starting MinIO tenant ...
while test (echo -n "Status: "; \
        kubectl get tenant minio1 \
        --ignore-not-found=true \
        --output=jsonpath="{.status.currentState}"; or echo "") \
        != "Status: Initialized"
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
helm repo add stackable https://repo.stackable.tech/repository/helm-dev
helm repo update stackable
helm install hive-operator stackable/hive-operator --version=0.3.0-nightly
helm install trino-operator stackable/trino-operator --version=0.1.0-nightly
