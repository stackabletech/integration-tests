use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use stackable_superset_crd::{SupersetCluster, SupersetVersion, APP_NAME};
use std::time::Duration;

const APP_NAME_LABEL: &str = "app.kubernetes.io/name";
const APP_INSTANCE_LABEL: &str = "app.kubernetes.io/instance";
const APP_VERSION_LABEL: &str = "app.kubernetes.io/version";

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<SupersetCluster> {
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

/// This returns a Superset custom resource.
pub fn build_superset_cluster(name: &str, version: &SupersetVersion) -> Result<SupersetCluster> {
    let spec = &formatdoc!(
        "
            apiVersion: superset.stackable.tech/v1alpha1
            kind: SupersetCluster
            metadata:
              name: {name}
            spec:
              version: {version}
              nodes:
                roleGroups:
                  default:
                    config:
                      credentialsSecret: simple-superset-credentials
        ",
        name = name,
        version = version.to_string()
    );

    Ok(serde_yaml::from_str(spec)?)
}
