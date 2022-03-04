#!/usr/bin/env python3

# To install the requirements run "pip install -r forward_services_locally_requirements.txt"

import argparse
import os
import re
import socket
import subprocess
import sys

from argparse import Namespace
from contextlib import closing
from kubernetes import client, config
from tabulate import tabulate

# We could further restrict the forwarded ports by converting this to a Map<str (regex), [int] ports>.
SERVICES_TO_EXPOSE = [
    r".*minio-external$",
    r".*minio.*-console$",
    r".*-kafka$",
    r".*-nifi$",
    r".*-superset-external$",
    r".*-trino-coordinator$",
    r".*-spark-master$",
    r".*-spark-history-server",
    r".*airflow-webserver$",
]

processes = []
forwarded_services = []

def check_args() -> Namespace:
  parser = argparse.ArgumentParser(
    description="This tool can be used to forward services of Stackable products to the local machine."
  )
  parser.add_argument('--namespace', '-n', required=False,
                      help='The namespace of the services to forward. As a default the current kubectl context will be used')
  parser.add_argument('--all-namespaces', '-a', action='store_true',
                      help='Forward services from all namespaces')
  parser.add_argument('--verbose', '-v', action='store_true',
                      help='Show stdout output of actual "kubectl port-forward" command')
  return parser.parse_args()

def main():
    args = check_args()

    config.load_kube_config()
    namespace = args.namespace or config.list_kube_config_contexts()[1]['context'].get("namespace", "default")
    k8s = client.CoreV1Api()

    if args.all_namespaces:
        services = k8s.list_service_for_all_namespaces()
    else:
        services = k8s.list_namespaced_service(namespace=namespace)

    for service in services.items:
        service_namespace = service.metadata.namespace
        service_name = service.metadata.name
        service_ports = [portSpec.port for portSpec in service.spec.ports]

        if shall_expose_service(service_name):
            for service_port in service_ports:
                forward_port(service_namespace, service_name, service_port, args.verbose)

    print(tabulate(forwarded_services, headers=['Namespace', 'Service', 'Port', 'URL'], tablefmt='psql'))
    print()

    for process in processes:
        process.wait()

def shall_expose_service(name) -> bool:
    for regex in SERVICES_TO_EXPOSE:
        if re.match(regex, name):
            return True
    return False

def find_free_port() -> int:
    with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as s:
        s.bind(('localhost', 0))
        return s.getsockname()[1]

def forward_port(service_namespace, service_name, service_port, verbose):
    local_port = find_free_port()
    command = ["kubectl", "-n", service_namespace, "port-forward", f"service/{service_name}", f"{local_port}:{service_port}"]
    if verbose:
        process = subprocess.Popen(command)
    else:
        process = subprocess.Popen(command, stdout=open(os.devnull, 'wb'))
    processes.append(process)
    forwarded_services.append([
        service_namespace,
        service_name,
        service_port,
        f"http://localhost:{local_port}"
    ])

def cleanup():
    if len(processes) > 0:
        print(f"Stopping {len(processes)} port-forwards")
        for process in processes:
            process.kill()
        processes.clear()

if __name__ == '__main__':
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        cleanup()
    finally:
        cleanup()
