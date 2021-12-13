#!/usr/bin/env sh

KIND_CLUSTER_CONFIG_FILE="kind-config.yaml"
KIND_CLUSTER_NAME="integration-tests"
HELM_DEV_REPO_NAME="stackable-dev"
HELM_DEV_REPO_URL="https://repo.stackable.tech/repository/helm-dev"
# Make PVC Size configurable since some CSI provider require minimal sizes higher than 10Mi.
MINIO_PVC_SIZE=${MINIO_PVC_SIZE-"10Mi"}
PYTHON=$(which python)
PIP=$(which pip)

create_kind_cluster() {

  local CLUSTER=$(kind get clusters 2>&1 | grep ${KIND_CLUSTER_NAME})
  if [ ! -z ${CLUSTER} ]; then
    echo Kind cluster ${KIND_CLUSTER_NAME} already running.
    return
  fi

  # Create Kubernetes cluster
  echo "
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
          node-labels: node=1,nodeType=druid-data
- role: worker
  kubeadmConfigPatches:
    - |
      kind: JoinConfiguration
      nodeRegistration:
        kubeletExtraArgs:
          node-labels: node=2
- role: worker
  kubeadmConfigPatches:
    - |
      kind: JoinConfiguration
      nodeRegistration:
        kubeletExtraArgs:
          node-labels: node=3
" | kind create cluster --name ${KIND_CLUSTER_NAME} --config -
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
    helm repo update ${HELM_REPO_NAME}
    if [ -z "${OPERATOR_VERSION}" ]; then
      helm install ${OPERATOR_NAME}-operator ${REPO}/${OPERATOR_NAME}-operator --devel
    else
      helm install ${OPERATOR_NAME}-operator ${REPO}/${OPERATOR_NAME}-operator --version=${OPERATOR_VERSION}
    fi
    install_dependencies ${OPERATOR_NAME} ${REPO}
  else
    if [ -z "${REUSE_CLUSTER}" ]; then
      echo Already running ${OPERATOR_NAME}-operator. You need to uninstall it first.
    else
      install_dependencies ${OPERATOR_NAME} ${REPO}
    fi
  fi
}


check_args() {
  echo $OPERATOR_NAME
  if [ -z "${OPERATOR_NAME}" ]; then
    echo ERROR: Missing operator name.
    help
    exit
  fi
}

help() {
  echo "Usage: ./create_test_cluster.sh -o operator-name [ -v operator-version] [-r]"
  echo "operator-name     : Can be one of: zookeeper, regorule, kafka, nifi, ..."
  echo "operator-version  : Optional Helm chart version."
  echo "-r                : Reuse existing kubernetes cluster."
}


install_dependencies() {
  local OPERATOR_NAME=$1
  local REPO=$2

  case ${OPERATOR_NAME} in
    hbase)
      install_dependencies_hbase
      ;;
    hive)
      install_dependencies_hive
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

install_dependencies_hive() {
  if [ -z "${PYTHON}" ] || [ -z "${PIP}" ]; then
    echo "ERROR: you need python *and* pip (version 3) to run the Hive tests!"
  else
    python3 -c 'import hive_metastore_client' >/dev/null 2>&1
    if [ $? -eq 1 ]; then
      pip3 install --user --requirement hive-operator/python/requirements.txt
    else
      echo Python requirements already installed.
    fi
  fi
}
install_dependencies_hbase() {
  install_operator zookeeper
  install_operator hdfs
}

install_dependencies_superset() {
  if [ -z "$(helm repo list | grep 'https://charts.bitnami.com/bitnami')" ]; then
    # Set bitnami repo for postgres charts
    helm repo add bitnami 'https://charts.bitnami.com/bitnami'
    helm repo update bitnami
  fi

  if [ -z "$(helm ls | grep 'postgresql' | awk '{print $1}')" ]; then
    helm install superset bitnami/postgresql \
      --set postgresqlUsername=superset \
      --set postgresqlPassword=superset \
      --set postgresqlDatabase=superset
  else
    echo Postgresql is already running.
  fi
}

install_dependencies_kafka() {
  install_operator zookeeper
  install_operator regorule
  install_operator opa
}

install_dependencies_trino() {
  if [ -z "$(helm repo list | grep minio)" ]; then
    # Set up S3
    helm repo add minio https://operator.min.io/
    helm repo update minio
  fi

  if [ -z "$(helm ls | grep minio-operator | awk '{print $1}')" ]; then
    # MinIO operator chart versions from 4.2.4 to 4.3.5 (which is the latest
    # at the time of writing) seem to be affected by
    # https://github.com/minio/operator/issues/904
    local minioOperatorChartVersion=4.2.3

    helm show values \
        --version $minioOperatorChartVersion \
        minio/minio-operator \
    | sed -e "
        /requestAutoCert:/ s/:.*/: false/
        /servers:/ s/:.*/: 1/g
        /size:/ s/:.*/: ${MINIO_PVC_SIZE}/" \
    | helm install \
        --version $minioOperatorChartVersion \
        --generate-name \
        --values - \
        minio/minio-operator

    echo Starting MinIO tenant ...
    while [ "$(kubectl get pod \
            --selector=v1.min.io/tenant=minio1 \
            --output=jsonpath='{range .items[*]}{.status.conditions[?(@.type=="Ready")].status}{end}')" \
            != "True" ]
    do
        sleep 2
    done

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

    local minioNodeIp=$(kubectl get pod \
        --selector='v1.min.io/tenant=minio1' \
        --output=jsonpath="{.items[0].status.hostIP}")
    local minioNodePort=$(kubectl get service minio-external \
        --output=jsonpath="{.spec.ports[0].nodePort}")

    export S3_ENDPOINT="http://$minioNodeIp:$minioNodePort"
    export S3_ACCESS_KEY=$(kubectl get secret minio1-secret \
            --output=jsonpath="{.data.accesskey}" | base64 --decode)
    export S3_SECRET_KEY=$(kubectl get secret minio1-secret \
            --output=jsonpath="{.data.secretkey}" | base64 --decode)

    echo !!!! Make sure the following variables are set in your environment before running
    echo !!!! the trino integration tests.
    echo export S3_ENDPOINT=${S3_ENDPOINT}
    echo export S3_ACCESS_KEY=${S3_ACCESS_KEY}
    echo export S3_SECRET_KEY=${S3_SECRET_KEY}

  else
    echo Minio is already running.
  fi

  # Deploy Hive and Trino operators
  install_operator hive
}

{
  while getopts ro:v:h flag
  do
      case "${flag}" in
          r) REUSE_CLUSTER="true";;
          o) OPERATOR_NAME=${OPTARG};;
          v) OPERATOR_VERSION=${OPTARG};;
          h) help;exit;;
          *) echo"unknown flag";help;exit;;
      esac
  done
  check_args
  if [ -z "${REUSE_CLUSTER}" ]; then
    create_kind_cluster
  fi
  install_operator "${OPERATOR_NAME}" "${OPERATOR_VERSION}"
}
