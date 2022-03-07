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
    r".*minio.*-console$": ["http-console"],
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
    r"alertmanager-operated": ["http-web"],
    r"prometheus-operated": ["http-web"],
    r"prometheus-operator-grafana": ["http-web"],
}

# Shell command to print additional infos like credentials
# The env variable SERVICE_NAME will be set to the service name
EXTRA_INFO = {
    r"prometheus-operator-grafana$": 'kubectl get secret prometheus-operator-grafana --template=\'user: {{index .data "admin-user" | base64decode}}, password: {{index .data "admin-password" | base64decode}}\'',
    r"minio.*-console$": 'kubectl get secret $(echo "$SERVICE_NAME" | sed "s/-console$/-secret/") --template=\'accesskey: {{index .data "accesskey" | base64decode}}, secretkey: {{index .data "secretkey" | base64decode}}\'',
    r".*-superset-external$": 'kubectl get secret $(echo "$SERVICE_NAME" | sed "s/-external$/-credentials/") --template=\'user: {{index .data "adminUser.username" | base64decode}}, password: {{index .data "adminUser.password" | base64decode}}\'',
}

k8s = None
k8s_node_ips = {}

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
  parser.add_argument('--dont-use-port-forward', '-d', action='store_true',
                      help='Dont use "kubectl port-forward", instead return the NodeIP and NodePort. This may cause problems as many services dont have the nodePorts attribute or services having multiple endpoints')
  parser.add_argument('--verbose', '-v', action='store_true',
                      help='In case of using port-forward show stdout output of "kubectl port-forward" command')
  return parser.parse_args()

def main():
    global k8s, k8s_nodes

    args = check_args()

    config.load_kube_config()
    namespace = args.namespace or config.list_kube_config_contexts()[1]['context'].get("namespace", "default")
    k8s = client.CoreV1Api()

    for node in k8s.list_node().items:
        node_name = node.metadata.name
        node_ip = None
        for address in node.status.addresses:
            if address.type in ('InternalIP', 'ExternalIP'):
                node_ip = address.address
                break
        if node_ip is None:
            raise Exception(f"Could not find a valid InternalIP or ExternalIP for node {node_name}")

        k8s_node_ips[node_name] = node_ip

    if args.all_namespaces:
        services = k8s.list_service_for_all_namespaces()
    else:
        services = k8s.list_namespaced_service(namespace=namespace)

    for service_spec in services.items:
        service_namespace = service_spec.metadata.namespace
        service_name = service_spec.metadata.name

        for port_spec in service_spec.spec.ports:
            service_port = port_spec.port
            service_port_name = port_spec.name
            service_node_port = port_spec.node_port or '<no nodePort attribute on service>'
            if shall_expose_service(service_name, service_port_name) or args.all_services:
                if args.dont_use_port_forward:
                    calculate_node_address(service_namespace, service_name, service_port, service_port_name, service_node_port)
                else:
                    forward_port(service_namespace, service_name, service_port, service_port_name, args.verbose)

    print(tabulate(forwarded_services, headers=['Namespace', 'Service', 'Port', 'Name', 'URL', 'Extra info'], tablefmt='psql'))
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
        f"http://localhost:{local_port}",
        get_extra_info(service_name),
    ])

def calculate_node_address(service_namespace, service_name, service_port, service_port_name, service_node_port):
    endpoint = k8s.read_namespaced_endpoints(namespace=service_namespace, name=service_name)
    node_name = endpoint.subsets[0].addresses[0].node_name
    node_ip = k8s_node_ips[node_name]

    forwarded_services.append([
        service_namespace,
        service_name,
        service_port,
        service_port_name,
        f"http://{node_ip}:{service_node_port}",
        get_extra_info(service_name),
    ])

def get_extra_info(service_name):
    command = None
    for regex, command_from_loop in EXTRA_INFO.items():
        if re.match(regex, service_name):
            command = command_from_loop
    if command is None:
        return ""
    else:
        complete_command = f"SERVICE_NAME={service_name} && {command}"
        return subprocess.check_output(complete_command, shell=True).decode('utf-8')

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
