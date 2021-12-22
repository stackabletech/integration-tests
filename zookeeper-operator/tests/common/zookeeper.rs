use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_zookeeper_crd::ZookeeperCluster;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<ZookeeperCluster> {
    TestCluster::new(
        &TestClusterOptions::new("zookeeper", "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns a ZooKeeper custom resource and the expected pod count.
pub fn build_zk_cluster(
    name: &str,
    version: &str,
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
                config:
                  adminPort: 8080
    ",
        name,
        version,
        replicas,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}
