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
  parser.add_argument('name')
  parser.add_argument('version', nargs='?')
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
  """ Checks whether Docker, Helm and Kind are installed (and running) """
  output = subprocess.run(['docker', 'info'], stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
  if output.returncode != 0:
    logging.error("This script uses docker, and it isn't running - please start docker and try again")
    logging.debug(bytes.decode(output.stdout))
    sys.exit(1)
  logging.debug("Docker seems to be running - continuing")

  helper_command_exists('kind')
  helper_command_exists('helm')


def create_kind_cluster():
  logging.debug(f"Checking whether kind cluster [{KIND_CLUSTER_NAME}] already exists")
  output = helper_execute(['kind', 'get', 'clusters'])
  if KIND_CLUSTER_NAME in output:
    logging.info(f"Kind cluster [{KIND_CLUSTER_NAME}] already running - continuing")
    return

  logging.info(f"Kind cluster [{KIND_CLUSTER_NAME}] missing - creating now")
  helper_execute(['kind', 'create', 'cluster', '--name', KIND_CLUSTER_NAME, '--config', '-'], KIND_CLUSTER_DEFINITION)
  logging.info('Successfully created kind cluster')


def install_stackable_operator(name: str, version: str = None):
  operator_name = f"{name}-operator"

  if version:
    args = [f"--version={version}"]
  else:
    args = ["--devel"]

  helper_install_helm_release(operator_name, HELM_DEV_REPO_NAME, HELM_DEV_REPO_URL, args)
  logging.info("Helm release was installed successfully")


def helper_add_helm_repo(name: str, url: str) -> str:
  """Adds Helm repository if it does not exist yet.

  :return: The name of the repository, might differ from the passed name if it did already exist
  """
  logging.debug(f"Checking whether Helm repository [{name}] already exists")
  output = json.loads(helper_execute(['helm', 'repo', 'list', '-o', 'json']))
  repo = next((item for item in output if item['url'] == url), None)

  if repo:
    logging.debug(f"Found existing repository [{repo['name']}] with URL [{repo['url']}]")
    helper_execute(['helm', 'repo', 'update', name])
    return repo['name']

  logging.info("Helm repository missing - adding now")
  helper_execute(['helm', 'repo', 'add', name, url])
  helper_execute(['helm', 'repo', 'update', name])
  logging.debug(f"Successfully added repository [{name}] with URL [{url}]")
  return name


def install_dependencies(name: str):
  # This requires Python 3.10 or above TODO Migrate to 3.9 compatible alternative
  match name:
    case "hbase":
      install_dependencies_hbase()
    case "hive":
      install_dependencies_hive()
    case "kafka":
      install_dependencies_kafka()
    case "opa":
      install_dependencies_opa()
    case "nifi" | "druid":
      logging.info(f"Installing dependencies for {name}")
      install_stackable_operator("zookeeper")
    case "superset":
      install_dependencies_superset()
    case "trino":
      install_dependencies_trino()


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


def install_dependencies_opa():
  logging.info("installing dependencies for OPA")
  install_stackable_operator("regorule")

def install_dependencies_superset():
  args = ['--set', 'postgresqlUsername=superset',
          '--set', 'postgresqlPassword=superset',
          '--set', 'postgresqlDatabase=superset']
  helper_install_helm_release("postgresql", "bitnami", "https://charts.bitnami.com/bitnami", args)


def install_dependencies_trino():
  repo = helper_add_helm_repo("minio", "https://operator.min.io/")
  helper_install_helm_release("minio", "minio", "https://operator.min.io/")
  # TODO - the row above is wrong because it's not capturing any of the customization from the bash version
  pass


def helper_install_helm_release(name: str, repo_name: str = None, repo_url: str = None, install_args: list = None):
  if repo_name and repo_url:
    repo_name = helper_add_helm_repo(repo_name, repo_url)

  release = helper_find_helm_release(name)
  if release:
    logging.info(f"Helm already running release with name [{release['name']}] and chart [{release['chart']}] - will not continue, you need to uninstall manually")
    sys.exit(1)

  logging.info("Installing Helm release now")
  args = ['helm', 'install', name, f"{repo_name}/{name}"]
  args = args + install_args
  helper_execute(args)
  logging.info("Helm release was installed successfully")


def helper_find_helm_release(name: str) -> dict:
  """ This tries to find a Helm release with an exact name like the passed in parameter OR with a chart that contains the passed in name.

  The returned object is a dict with these fields in Helm 3.7 (or None if not found): name, namespace, revision, updated, status, chart, app_version
  """
  logging.debug(f"Looking for helm release with chart or name of [{name}]")
  output = json.loads(helper_execute(['helm', 'ls', '-o', 'json']))
  return next((item for item in output if item['name'] == name or name in item['chart']), None)


def helper_command_exists(command: str):
  """ This will check (using `which`) whether the given command exists.
  If not we'll exit the program.
  """
  if shutil.which(command) is None:
    logging.error(f"This script uses '{command}' but it could not be found - please install and try again")
    sys.exit(1)
  logging.debug(f"'{command}' seems to be available - continuing")


def helper_execute(args, stdin: str = None) -> str:
  output = subprocess.run(
    args,
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
    input=stdin,
    text=True
  )

  args_string = " ".join(args)

  if output.returncode == 0:
    logging.debug('Successfully ran: ' + args_string)
    if output.stdout:
      logging.debug("Output of the program:")
      logging.debug(output.stdout.rstrip("\n"))
    return output.stdout
  else:
    logging.error('Error running: ' + args_string)
    if output.stdout:
      logging.debug("Output of the program:")
      logging.error(output.stdout.rstrip("\n"))
    sys.exit(1)


def main() -> int:
  args = check_args()
  check_prerequisites()
  create_kind_cluster()
  install_stackable_operator(args.name, args.version)
  install_dependencies(args.name)
  return 0


if __name__ == '__main__':
  sys.exit(main())
