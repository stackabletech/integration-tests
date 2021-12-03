use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_opa_crd::{OpenPolicyAgent, APP_NAME};

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<OpenPolicyAgent> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
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
                  regoRuleReference: regorule-service:3030/opa/v1
    ",
        name, version, replicas,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}
