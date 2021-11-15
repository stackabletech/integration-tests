use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use stackable_opa_crd::{OpenPolicyAgent, APP_NAME};
use std::time::Duration;

const APP_NAME_LABEL: &str = "app.kubernetes.io/name";
const APP_INSTANCE_LABEL: &str = "app.kubernetes.io/instance";
const APP_VERSION_LABEL: &str = "app.kubernetes.io/version";

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<OpenPolicyAgent> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        // TODO: the app, instance and version labels should be recovered from kube-rs / k8s-openapi
        //    independent crate in operator-rs
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// This returns an OPA custom resource and the expected pod count.
pub fn build_opa_cluster(
    name: &str,
    version: &str,
    replicas: usize,
) -> Result<(OpenPolicyAgent, usize)> {
    let spec = &format!(
        "
        apiVersion: opa.stackable.tech/v1alpha1
        kind: OpenPolicyAgent
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
                config:
                  port: 8181
                  repoRuleReference: no_reference
    ",
        name, version, replicas,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}
