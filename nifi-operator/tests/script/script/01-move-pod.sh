#!/usr/bin/env bash
# The purpose of this command is to cordon the node which is currently running the pod that has been elected controller
# of the NiFi cluster.
# The pod name of the controller has been retrieved in 04-assert.py and written to the temp file 'controller'
# We retrieve the pod from k8s and look up the node with yq
echo "Cordon node running controller"
kubectl cordon $(kubectl --namespace kuttl-test-fresh-egret get pod $(cat tests/kuttl/smoke/controller) -o yaml | yq eval '.spec.nodeName' -)

# Now that the node is cordoned off we can delete the pod and it will be assigned to a different node
echo "Restart controller pod"
kubectl --namespace $NAMESPACE delete pod $(cat tests/kuttl/smoke/controller)