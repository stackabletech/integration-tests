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
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// This returns a ZooKeeper custom resource and the expected pod count.
pub fn build_zk_cluster(
    name: &str,
    version: &ZookeeperVersion,
    replicas: usize,
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
    ",
        name,
        version.to_string(),
        replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

/// This returns a ZooKeeper custom resource and the expected pod count (1). We use labels
/// for host_name and assign it to the node_ids provided by test-dev-cluster.
/// This creates 1 ZooKeeper server with a user defined client and metrics port.
pub fn build_zk_cluster_with_metrics_and_client_port(
    name: &str,
    version: &ZookeeperVersion,
    replicas: usize,
    client_port: u16,
    metrics_port: u16,
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
                  clientPort: {}
                  metricsPort: {}
    ",
        name,
        version.to_string(),
        replicas,
        client_port,
        metrics_port,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}
