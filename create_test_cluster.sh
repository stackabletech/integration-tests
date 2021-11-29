#!/usr/bin/env sh

OPERATOR_NAME=${1}
OPERATOR_VERSION=${2}

KIND_CLUSTER_CONFIG_FILE="kind-config.yaml"
KIND_CLUSTER_NAME="integration-tests"
HELM_DEV_REPO_NAME="stackable-dev"
HELM_DEV_REPO_URL="https://repo.stackable.tech/repository/helm-dev"

create_kind_cluster() {

  local CLUSTER=$(kind get clusters 2>&1 | grep ${KIND_CLUSTER_NAME})
  if [ ! -z ${CLUSTER} ]; then
    echo Kind cluster ${KIND_CLUSTER_NAME} already running.
    return
  fi

  # Write cluster config file
  tee ${KIND_CLUSTER_CONFIG_FILE} > /dev/null << KIND_CONFIG
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
- role: worker
  kubeadmConfigPatches:
    - |
      kind: JoinConfiguration
      nodeRegistration:
        kubeletExtraArgs:
          node-labels: "node=1"
- role: worker
  kubeadmConfigPatches:
    - |
      kind: JoinConfiguration
      nodeRegistration:
        kubeletExtraArgs:
          node-labels: "node=2"
- role: worker
  kubeadmConfigPatches:
    - |
      kind: JoinConfiguration
      nodeRegistration:
        kubeletExtraArgs:
          node-labels: "node=3"
KIND_CONFIG

  # Create Kubernetes cluster
  kind create cluster --name ${KIND_CLUSTER_NAME} --config ${KIND_CLUSTER_CONFIG_FILE}
}

install_operator() {
  local REPO=$(helm repo list | grep ${HELM_DEV_REPO_URL} | awk '{print $1}')
  if [ -z "${REPO}" ]; then
    helm repo add ${HELM_DEV_REPO_NAME} ${HELM_DEV_REPO_URL}
    REPO=${HELM_DEV_REPO_NAME}
  fi

  local HELM_RELEASE=$(helm ls | grep ${OPERATOR_NAME}-operator | awk '{print $1}')
  if [ -z "${HELM_RELEASE}" ]; then
    helm repo update ${HELM_REPO_NAME}
    helm install ${OPERATOR_NAME}-operator ${REPO}/${OPERATOR_NAME}-operator --version=${OPERATOR_VERSION} --devel
  else
    echo Already running ${OPERATOR_NAME}-operator. You need to uninstall it first.
  fi
}

check_args() {
  if [ -z "${OPERATOR_NAME}" ] || [ -z "${OPERATOR_VERSION}" ]; then
    echo ERROR: Missing argument
    help
    exit
  fi
}

help() {
  echo "Usage: ./create_test-cluster.sh operator-name operator-version"
  echo "operator-name     : Can be one of: zookeeper, regorule, kafka, nifi, ..."
  echo "operator-version  : Helm chart version."
}

{
  check_args
  create_kind_cluster
  install_operator
}


## Set up S3
#helm repo add minio https://operator.min.io/
#helm repo update minio
#helm install \
#    --namespace minio-operator \
#    --create-namespace \
#    --generate-name \
#    --set tenants[0].name=minio1 \
#    --set tenants[0].namespace=default \
#    --set tenants[0].pools[0].servers=1 \
#    --set tenants[0].pools[0].size=10Mi \
#    --set tenants[0].pools[0].storageClassName=standard \
#    --set tenants[0].secrets.enabled=true \
#    --set tenants[0].secrets.name=minio1-secret \
#    --set tenants[0].secrets.accessKey=minio \
#    --set tenants[0].secrets.secretKey=minio123 \
#    --set tenants[0].certificate.requestAutoCert=false \
#    minio/minio-operator
#
#echo Waiting 90 seconds for MinIO service to be up and running ...
#sleep 90
#
#echo "
#apiVersion: v1
#kind: Service
#metadata:
#  name: minio-external
#spec:
#  type: NodePort
#  selector:
#    v1.min.io/tenant: minio1
#  ports:
#    - port: 80
#      targetPort: 9000
#" | kubectl apply -f -
#
#set minioNodeIp \
#    (kubectl get pod \
#        --selector='v1.min.io/tenant=minio1' \
#        --output=jsonpath="{.items[0].status.hostIP}")
#set minioNodePort \
#    (kubectl get service minio-external \
#        --output=jsonpath="{.spec.ports[0].nodePort}")
# 
#set -Ux S3_ENDPOINT "http://$minioNodeIp:$minioNodePort"
#set -Ux S3_ACCESS_KEY \
#    (kubectl get secret minio1-secret \
#        --output=jsonpath="{.data.accesskey}" | base64 --decode)
#set -Ux S3_SECRET_KEY \
#    (kubectl get secret minio1-secret \
#        --output=jsonpath="{.data.secretkey}" | base64 --decode)
#
## Deploy Hive and Trino operators
#helm repo add stackable https://repo.stackable.tech/repository/helm-dev
#helm repo update stackable
#helm install hive-operator stackable/hive-operator --version=0.3.0-nightly
#helm install trino-operator stackable/trino-operator --version=0.1.0-nightly

