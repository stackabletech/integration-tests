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

# An empty list of port names to expose means expose all ports of that service
SERVICES_TO_EXPOSE = {
    r".*minio$": ["http-minio"],
    r".*minio-hl$": ["http-minio"],
    r".*minio-console$": ["http-console"],
    r".*opa": ["http", "https"],
    r".*druid-(broker|coordinator|historical|middlemanager|router)": ["http", "https"],
    r".*hbase-(master|regionserver|restserver)": ["ui"],
    r".*spark-master": ["http", "https"],
    r".*spark-slave": ["http", "https"],
    r".*spark-history-server": ["http", "https"],
    r".*trino-(coordinator|worker)": ["http", "https"],
    r".*nifi": ["http", "https"],
    r".*airflow-webserver": ["airflow"],
    r".*superset": ["superset"],
    r".*simple-hdfs-namenode-.*$(?<!-[0-9])(?<!-[0-9][0-9])(?<!-[0-9][0-9][0-9])": ["http", "https"],
    r".*simple-hdfs-datanode-.*$(?<!-[0-9])(?<!-[0-9][0-9])(?<!-[0-9][0-9][0-9])": ["http", "https"],
    r".*simple-hdfs-journalnode-.*$(?<!-[0-9])(?<!-[0-9][0-9])(?<!-[0-9][0-9][0-9])": ["http", "https"],
}

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
  parser.add_argument('--all-services', action='store_true',
                      help='Forward all services regardles of the name or the exposed ports')
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

        for portSpec in service.spec.ports:
            service_port = portSpec.port
            service_port_name = portSpec.name
            if shall_expose_service(service_name, service_port_name) or args.all_services:
                forward_port(service_namespace, service_name, service_port, service_port_name, args.verbose)

    print(tabulate(forwarded_services, headers=['Namespace', 'Service', 'Port', 'Name', 'URL'], tablefmt='psql'))
    print()

    for process in processes:
        process.wait()

def shall_expose_service(service_name, service_port_name) -> bool:
    for regex, port_names_to_expose in SERVICES_TO_EXPOSE.items():
        if re.match(regex, service_name) \
            and (service_port_name in port_names_to_expose or len(port_names_to_expose) == 0):
            return True
    return False

def find_free_port() -> int:
    with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as s:
        s.bind(('localhost', 0))
        return s.getsockname()[1]

def forward_port(service_namespace, service_name, service_port, service_port_name, verbose):
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
        service_port_name,
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
