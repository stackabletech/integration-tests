import argparse
import importlib
import json
import logging
import shutil
import sys
import subprocess
from argparse import Namespace

KIND_CLUSTER_NAME="integration-tests"

HELM_DEV_REPO_NAME="stackable-dev"
HELM_DEV_REPO_URL="https://repo.stackable.tech/repository/helm-dev"


KIND_CLUSTER_DEFINITION = """
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
"""


def check_args() -> Namespace:
  parser = argparse.ArgumentParser()
  parser.add_argument('--operator', '-o', help='The Stackable operator to install')
  parser.add_argument('--version', '-v', required=False, help='The version of the operator to install, if left empty it will install the latest development version')
  parser.add_argument('--kind', '-k', action='store_true', required=False, help="When set we'll automatically create a 4 node kind cluster")
  parser.add_argument('--debug', action='store_true', required=False)
  args = parser.parse_args()

  log_level = 'DEBUG' if args.debug else 'INFO'
  logging.basicConfig(
    level=log_level,
    format='%(asctime)s %(levelname)s: %(message)s',
    stream=sys.stdout
  )

  return args


def check_prerequisites():
  """ Checks whether Helm is installed"""
  helper_command_exists('helm')


def create_kind_cluster(name: str):
  """ Creates a kind cluster with four nodes and the given name if it doesn't exist already"""
  helper_command_exists('kind')
  helper_check_docker_running()

  logging.debug(f"Checking whether kind cluster [{name}] already exists")
  output = helper_execute(['kind', 'get', 'clusters']).splitlines()
  if name in output:
    logging.info(f"Kind cluster [{name}] already running - continuing")
    return

  logging.info(f"Kind cluster [{name}] missing - creating now")
  helper_execute(['kind', 'create', 'cluster', '--name', name, '--config', '-'], KIND_CLUSTER_DEFINITION)
  logging.info(f'Successfully created kind cluster [{name}]')


def check_kubernetes_available():
  logging.info("Checking if Kubernetes is available")
  helper_execute(['kubectl', 'cluster-info'])
  logging.debug("Successfully tested for Kubernetes, seems to be available")


def install_stackable_operator(name: str, version: str = None):
  """ This installs a Stackable Operator release in Helm.

  It makes sure that the proper repository is installed and install either a specific version or the latest development version
  """
  operator_name = f"{name}-operator"

  if version:
    args = [f"--version={version}"]
  else:
    args = ["--devel"]

  helper_install_helm_release(operator_name, operator_name, HELM_DEV_REPO_NAME, HELM_DEV_REPO_URL, args)
  install_dependencies(name)


def helper_check_docker_running():
  """Check if Docker is running, exit the program if not"""
  output = subprocess.run(['docker', 'info'], stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
  if output.returncode != 0:
    logging.error("This script uses docker, and it isn't running - please start docker and try again")
    logging.debug(output.stdout)
    sys.exit(1)
  logging.debug("Docker seems to be running - continuing")


def helper_add_helm_repo(name: str, url: str) -> str:
  """Adds Helm repository if it does not exist yet (it looks for a repository with the same URL, not name).

  An `update` command will also be run in either case.

  :return: The name of the repository, might differ from the passed name if it did already exist
  """
  logging.debug(f"Checking whether Helm repository [{name}] already exists")
  output = json.loads(helper_execute(['helm', 'repo', 'list', '-o', 'json']))
  repo = next((item for item in output if item['url'] == url), None)

  if repo:
    logging.debug(f"Found existing repository [{repo['name']}] with URL [{repo['url']}]")
    helper_execute(['helm', 'repo', 'update', name])
    return repo['name']

  logging.info(f"Helm repository [{name}] (URL {url}) missing - adding now")
  helper_execute(['helm', 'repo', 'add', name, url])
  helper_execute(['helm', 'repo', 'update', name])
  logging.debug(f"Successfully added repository [{name}] with URL [{url}]")
  return name


def install_dependencies(name: str):
  # In Python 3.10 this could have been a match-case statement
  options = {
    "druid": install_dependencies_druid,
    "hbase": install_dependencies_hbase,
    "hive": install_dependencies_hive,
    "kafka": install_dependencies_kafka,
    "nifi": install_dependencies_nifi,
    "opa": install_dependencies_opa,
    "superset": install_dependencies_superset,
    "trino": install_dependencies_trino
  }
  if name in options:
    options[name]()


def install_dependencies_druid():
  logging.info("Installing dependencies for Druid")
  install_stackable_operator("zookeeper")


def install_dependencies_hbase():
  logging.info("Installing dependencies for HBase")
  install_stackable_operator("zookeeper")
  install_stackable_operator("hdfs")


def install_dependencies_hive():
  logging.info("Checking prerequisites and installing dependencies for Hive")
  helper_command_exists('python')
  helper_command_exists('pip')

  logging.debug("Checking whether the Python Hive requirements have been installed")
  metastore_client_spec = importlib.util.find_spec('hive_metastore_client')
  if metastore_client_spec is None:
    logging.info("Python requirements for Hive are missing - installing now")
    helper_execute([sys.executable, '-m', 'pip', 'install', '--user', '--requirement', 'hive-operator/python/requirements.txt'])
  else:
    logging.debug("Python requirements for Hive seem to be installed already")


def install_dependencies_kafka():
  logging.info("Installing dependencies for Kafka")
  install_stackable_operator("zookeeper")
  install_stackable_operator("regorule")
  install_stackable_operator("opa")


def install_dependencies_nifi():
  logging.info("Installing dependencies for NiFi")
  install_stackable_operator("zookeeper")


def install_dependencies_opa():
  logging.info("Installing dependencies for OPA")
  install_stackable_operator("regorule")


def install_dependencies_superset():
  logging.info("Installing dependencies for Superset")
  args = ['--set', 'postgresqlUsername=superset',
          '--set', 'postgresqlPassword=superset',
          '--set', 'postgresqlDatabase=superset']
  helper_install_helm_release("superset-postgresql", "postgresql", "bitnami", "https://charts.bitnami.com/bitnami", args)


def install_dependencies_trino():
  repo = helper_add_helm_repo("minio", "https://operator.min.io/")
  release = helper_find_helm_release("minio-operator", "minio-operator")
  if release:
    logging.info(f"MinIO already running release with name [{release['name']}] and chart [{release['chart']}] - skipping installation")
    return

  helper_install_helm_release("minio", "minio", "https://operator.min.io/")
  # TODO - the line above is wrong because it's not capturing any of the customization from the bash version
  pass


def helper_install_helm_release(name: str, chart_name: str, repo_name: str = None, repo_url: str = None, install_args: list = None):
  if repo_name and repo_url:
    repo_name = helper_add_helm_repo(repo_name, repo_url)

  release = helper_find_helm_release(name, chart_name)
  if release:
    logging.info(f"Helm already running release with name [{release['name']}] and chart [{release['chart']}] - will not take any further action for this release")
    return
  else:
    logging.debug(f"No Helm release with the name {name} found")

  logging.info(f"Installing Helm release [{name}] from chart [{chart_name}] now")
  args = ['helm', 'install', name, f"{repo_name}/{chart_name}"]
  args = args + install_args
  helper_execute(args)
  logging.info("Helm release was installed successfully")


def helper_find_helm_release(name: str, chart_name: str) -> dict:
  """ This tries to find a Helm release with an _exact_ name like the passed in parameter OR with a chart that _contains_ the passed in chart name.

  The returned object is a dict with these fields in Helm 3.7 (or None if not found): name, namespace, revision, updated, status, chart, app_version
  """
  logging.debug(f"Looking for helm release with chart or name of [{name}]")
  output = json.loads(helper_execute(['helm', 'ls', '-o', 'json']))
  return next((item for item in output if item['name'] == name or chart_name in item['chart']), None)


def helper_command_exists(command: str):
  """ This will check (using `which`) whether the given command exists.
  If not we'll exit the program.
  """
  if shutil.which(command) is None:
    logging.error(f"This script uses '{command}' but it could not be found - please install and try again")
    sys.exit(1)
  logging.debug(f"'{command}' seems to be available - continuing")


def helper_execute(args, stdin: str = None) -> str:
  """ This will execute the passed in program and exit the program if it failed.

  In case of a failure or if debug is enabled it will also print the stderr and stdout of the program, otherwise it'll be silen
  """
  args_string = " ".join(args)
  logging.debug("Running now: " + args_string)
  output = subprocess.run(
    args,
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
    input=stdin,
    text=True
  )

  if output.returncode == 0:
    logging.debug('Successfully ran: ' + args_string)
    if output.stdout:
      logging.debug("Output of the program:")
      logging.debug("\n>>>>>>>>>>>>>>>>>>>>>>>\n" + output.stdout.strip("\n") + "\n<<<<<<<<<<<<<<<<<<<<<<<")
    return output.stdout
  else:
    logging.error('Error running: ' + args_string)
    if output.stdout:
      logging.error("Output of the program:")
      logging.error("\n>>>>>>>>>>>>>>>>>>>>>>>\n" + output.stdout.strip("\n") + "\n<<<<<<<<<<<<<<<<<<<<<<<")
    sys.exit(1)


def main() -> int:
  args = check_args()
  check_prerequisites()
  if args.kind:
    create_kind_cluster(KIND_CLUSTER_NAME)
  check_kubernetes_available()
  install_stackable_operator(args.operator, args.version)
  logging.info(f"Successfully installed operator for {args.operator}")
  return 0


if __name__ == '__main__':
  sys.exit(main())
