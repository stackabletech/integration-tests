#!/usr/bin/env python3

# Requirements: pip install kubernetes tabulate

import re
import socket
import subprocess
import sys

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

def main():
    config.load_kube_config()
    current_namespace = config.list_kube_config_contexts()[1]['context'].get("namespace", "default")
    k8s = client.CoreV1Api()

    services = k8s.list_namespaced_service(namespace=current_namespace)
    for service in services.items:
        service_name = service.metadata.name
        service_ports = [portSpec.port for portSpec in service.spec.ports]

        if shall_expose_service(service_name):
            for service_port in service_ports:
                forward_port(service_name, service_port)

    print(tabulate(forwarded_services, headers=['Service', 'Service port', 'URL'], tablefmt='psql'))
    print()

    for process in processes:
        process.wait()

def shall_expose_service(name):
    for regex in SERVICES_TO_EXPOSE:
        if re.match(regex, name):
            return True
    return False

def find_free_port():
    with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as s:
        s.bind(('localhost', 0))
        return s.getsockname()[1]

def forward_port(service_name, service_port):
    local_port = find_free_port()
    processes.append(subprocess.Popen(["kubectl", "port-forward", f"service/{service_name}", f"{local_port}:{service_port}"]))
    forwarded_services.append([
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
