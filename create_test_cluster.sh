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
          node-labels: "node=1,nodeType=druid-data"
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
  local OPERATOR_NAME=$1
  local OPERATOR_VERSION=$2

  local REPO=$(helm repo list | grep ${HELM_DEV_REPO_URL} | awk '{print $1}')
  if [ -z "${REPO}" ]; then
    helm repo add ${HELM_DEV_REPO_NAME} ${HELM_DEV_REPO_URL}
    REPO=${HELM_DEV_REPO_NAME}
  fi

  local HELM_RELEASE=$(helm ls | grep ${OPERATOR_NAME}-operator | awk '{print $1}')
  if [ -z "${HELM_RELEASE}" ]; then
    # helm repo update ${HELM_REPO_NAME}
    if [ -z "${OPERATOR_VERSION}" ]; then
      helm install ${OPERATOR_NAME}-operator ${REPO}/${OPERATOR_NAME}-operator --devel
    else
      helm install ${OPERATOR_NAME}-operator ${REPO}/${OPERATOR_NAME}-operator --version=${OPERATOR_VERSION} --devel
    fi
    install_dependencies ${OPERATOR_NAME} ${REPO}
  else
    echo Already running ${OPERATOR_NAME}-operator. You need to uninstall it first.
  fi
}

check_args() {
  if [ -z "${OPERATOR_NAME}" ]; then
    echo ERROR: Missing operator name.
    help
    exit
  fi
}

help() {
  echo "Usage: ./create_test-cluster.sh operator-name [operator-version]"
  echo "operator-name     : Can be one of: zookeeper, regorule, kafka, nifi, ..."
  echo "operator-version  : Optional Helm chart version."
}


install_dependencies() {
  local OPERATOR_NAME=$1
  local REPO=$2

  case ${OPERATOR_NAME} in
    hbase)
      install_dependencies_hbase
      ;;
    kafka)
      install_dependencies_kafka
      ;;
    opa)
      install_operator regorule
      ;;
    nifi|druid)
      install_operator zookeeper
      ;;
    superset)
      install_dependencies_superset
      ;;
    trino)
      install_dependencies_trino
      ;;
    *)
      ;;
  esac
}

install_dependencies_hbase() {
  install_operator zookeeper
  install_operator hdfs
}

install_dependencies_trino() {
  install_operator hive
  install_operator regorule
  install_operator opa
}

install_dependencies_kafka() {
  install_operator zookeeper
  install_operator regorule
  install_operator opa
}

install_dependencies_superset() {
  if [ -z "$(helm repo list | grep minio)" ]; then
    # Set up S3
    helm repo add minio https://operator.min.io/
    helm repo update minio
  fi

  if [ -z "$(helm ls --namespace minio-operator | grep minio-operator | awk '{print $1}')" ]; then
    helm install \
        --namespace minio-operator \
        --create-namespace \
        --generate-name \
        --set 'tenants[0].name=minio1' \
        --set 'tenants[0].namespace=default' \
        --set 'tenants[0].pools[0].servers=1' \
        --set 'tenants[0].pools[0].size=10Mi' \
        --set 'tenants[0].pools[0].storageClassName=standard' \
        --set 'tenants[0].secrets.enabled=true' \
        --set 'tenants[0].secrets.name=minio1-secret' \
        --set 'tenants[0].secrets.accessKey=minio' \
        --set 'tenants[0].secrets.secretKey=minio123' \
        --set 'tenants[0].certificate.requestAutoCert=false' \
        minio/minio-operator

    echo Waiting 90 seconds for MinIO service to be up and running ...
    sleep 90

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

    export minioNodeIp=$(kubectl get pod \
            --selector='v1.min.io/tenant=minio1' \
            --output=jsonpath="{.items[0].status.hostIP}")
    export  minioNodePort=$(kubectl get service minio-external \
            --output=jsonpath="{.spec.ports[0].nodePort}")
     
    export S3_ENDPOINT="http://$minioNodeIp:$minioNodePort"
    export S3_ACCESS_KEY=$(kubectl get secret minio1-secret \
            --output=jsonpath="{.data.accesskey}" | base64 --decode)
    export S3_SECRET_KEY=$(kubectl get secret minio1-secret \
            --output=jsonpath="{.data.secretkey}" | base64 --decode)
  else
    echo Minio is already running.
  fi

  # Deploy Hive and Trino operators
  install_operator hive
  install_operator trino
}

{
  check_args
  create_kind_cluster
  install_operator ${OPERATOR_NAME} ${OPERATOR_VERSION}
}


