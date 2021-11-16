use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_zookeeper_crd::{ZookeeperCluster, ZookeeperVersion, APP_NAME};
use std::time::Duration;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<ZookeeperCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(60),
        },
    )
}

/// This returns a ZooKeeper custom resource and the expected pod count.
pub fn build_zk_cluster(
    name: &str,
    version: &ZookeeperVersion,
    replicas: usize,
    admin_port: Option<i32>,
    client_port: Option<i32>,
) -> Result<(ZookeeperCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: zookeeper.stackable.tech/v1alpha1
        kind: ZookeeperCluster
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: {}
                config:
                  adminPort: {}
                  clientPort: {}
    ",
        name,
        version.to_string(),
        replicas,
        admin_port.unwrap_or(8080),
        client_port.unwrap_or(2181),
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

/// This returns a ZooKeeper custom resource with metrics enabled and the expected pod count.
pub fn build_zk_cluster_with_metrics(
    name: &str,
    version: &ZookeeperVersion,
    replicas: usize,
    admin_port: Option<i32>,
    client_port: Option<i32>,
    metrics_port: Option<i32>,
) -> Result<(ZookeeperCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: zookeeper.stackable.tech/v1alpha1
        kind: ZookeeperCluster
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: {}
                config:
                  adminPort: {}
                  clientPort: {}
                  metricsPort: {}
    ",
        name,
        version.to_string(),
        replicas,
        admin_port.unwrap_or(8080),
        client_port.unwrap_or(2181),
        metrics_port.unwrap_or(9505),
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}
