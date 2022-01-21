#!/usr/bin/env python

import os
import requests
import json
##
## TOKEN=$(
##
##        curl -k
##    'https://test-nifi.elia.svc.cluster.local:8443/nifi-api/access/token'
##    -H 'Accept-Encoding: gzip, deflate, br'
##    -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8'
##    --data 'username=admin&password=supersecretpassword'
##    --compressed)


def get_token(username, password):

    headers = {
        'content-type': 'application/x-www-form-urlencoded; charset=UTF-8',
    }

    data = {'username': username, 'password': password}

    # TODO: handle actual errors when connecting properly
    response = requests.post('https://test-nifi-node-default-1.test-nifi-node-default.elia.svc.cluster.local:8443/nifi-api/access/token', headers=headers, data=data, verify='cacert.pem')

    if response.status_code != 201:
        print("Failed to retrieve token!")
        exit(-1)

    token = response.content.decode('utf-8')
    return "Bearer " + token

token = get_token('admin', 'supersecretpassword')

headers = {'Authorization': token}
cluster = requests.get('https://test-nifi-node-default-1.test-nifi-node-default.elia.svc.cluster.local:8443/nifi-api/controller/cluster', headers=headers, verify='cacert.pem')
cluster_data = json.loads(cluster.content.decode('utf-8'))

print(json.dumps(cluster_data['cluster']['nodes'], indent=4))





