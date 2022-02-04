#!/usr/bin/env bash
# The purpose of this command is to cordon the node which is currently running the pod that has been elected controller
# of the NiFi cluster.
# The pod name of the controller has been retrieved in 04-assert.py and written to the temp file 'controller'
# We retrieve the pod from k8s and look up the node with yq
CONTROLLER=$(cat controller)
echo "Found controller pod: $CONTROLLER"

CONTROLLER_NODE=$(kubectl --namespace $NAMESPACE get pod $(cat controller) -o yaml | yq eval '.spec.nodeName' -)

echo "Cordon node $CONTROLLER_NODE which is running controller"
kubectl cordon $CONTROLLER_NODE

# Now that the node is cordoned off we can delete the pod and it will be assigned to a different node
echo "Restart controller pod"
kubectl --namespace $NAMESPACE delete pod $CONTROLLER