use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use stackable_hive_crd::{HiveCluster, APP_NAME};
use std::time::Duration;

const APP_NAME_LABEL: &str = "app.kubernetes.io/name";
const APP_INSTANCE_LABEL: &str = "app.kubernetes.io/instance";
const APP_VERSION_LABEL: &str = "app.kubernetes.io/version";

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<HiveCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        // TODO: the app, instance and version labels should be recovered from kube-rs / k8s-openapi
        //    independent crate in operator-rs
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(180),
        },
    )
}

/// This returns a Hive custom resource and the expected pod count.
pub fn build_hive_cluster(
    name: &str,
    version: &str,
    database_path: &str,
    replicas: usize,
) -> Result<(HiveCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: hive.stackable.tech/v1alpha1
        kind: HiveCluster
        metadata:
          name: {}
        spec:
          version: {}
          metastore:
            roleGroups:
              default:
                selector:
                  matchExpressions:
                    - operator: In
                      key: kubernetes.io/arch
                      values:
                        - stackable-linux
                replicas: {}
                config:
                  javaHome: /usr/lib/jvm/java-11-openjdk-amd64/
                  metricsPort: 11111
                  database:
                    connString: jdbc:derby:;databaseName={};create=true
                    user: APP
                    password: mine
                    dbType: derby

        ",
        name, version, replicas, database_path
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}
