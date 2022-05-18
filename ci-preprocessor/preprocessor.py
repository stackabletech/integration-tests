import os
import os.path
import datetime
import time
import sys
import yaml
import requests
import re
import subprocess
import hiyapyco

platforms = None
tests = None

def check_prerequisites():
    """ Checks the prerequisites of this script and fails if they are not satisfied. """
    if not os.path.isdir("/target"):
        print("Error Please supply target folder as volume '/target.'")
        exit(1)
    if not os.path.isfile("/platforms.yaml"):
        print("Error Please supply platform definition file as volume '/platforms.yaml.'")
        exit(1)
    if not os.path.isfile("/tests.yaml"):
        print("Error Please supply test definition file as volume '/tests.yaml.'")
        exit(1)
    if not 'TEST_NAME' in os.environ:
        print("Error: Please supply TEST_NAME as an environment variable.")
        exit(1)

def clean_target():
    os.system('rm -rf /target/*')

def read_catalogs():
    global platforms
    global tests
    platforms = hiyapyco.load("/platforms.yaml")
    tests = hiyapyco.load("/tests.yaml")

def get_platform(name):
    global platforms
    return next(filter(lambda x: x["metadata"]["name"]==name, platforms), None)

def get_test(name):
    global tests
    return next(filter(lambda x: x["name"]==name, tests), None)

def get_platform_overlay(test):
    if not "platform_overlay" in test:
        return None
    return test["platform_overlay"]

def mergeYaml(baseYaml, yamlToMerge):
    """ both as string! """
    return hiyapyco.load([baseYaml, yamlToMerge], method=hiyapyco.METHOD_MERGE)

def write_cluster_definition(cluster_definition, folder):
    os.makedirs(folder)
    with open (f"{folder}cluster.yaml", "w") as f:
        f.write(cluster_definition)
        f.close()

def yamlToString(yaml):
    return hiyapyco.dump(yaml, default_flow_style=False, width=1000)

if __name__ == "__main__":

    check_prerequisites()
    clean_target()
    read_catalogs()

    # read the test
    test_name = os.environ["TEST_NAME"]
    test = get_test(test_name)
    if not test:
        print(f"The test '{test_name}' does not exist.")
        exit(1)

    # read the platform
    if not "platform" in test:
        print(f"The platform for the test '{test_name}' is not specified.")
        exit(1)
    platform_name = test["platform"]
    platform_definition = get_platform(platform_name)
    if not platform_definition:
        print(f"The platform named '{platform_name}' specifiec in test '{test_name}' does not exist.")
        exit(1)

    cluster_definition = platform_definition

    # apply the overlay if present
    platform_overlay = get_platform_overlay(test)
    if platform_overlay:
        cluster_definition = mergeYaml(yamlToString(cluster_definition), platform_overlay)

    # write the cluster definition to disk
    write_cluster_definition(yamlToString(cluster_definition), f"/target/{test_name}/")


