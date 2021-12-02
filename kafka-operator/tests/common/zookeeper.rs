use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::time::Duration;

use anyhow::Result;
use stackable_zookeeper_crd::{ZookeeperCluster, ZookeeperVersion, APP_NAME};

use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use integration_test_commons::test::prelude::formatdoc;

pub fn build_zk_test_cluster(app_name: &str) -> Result<TestCluster<ZookeeperCluster>> {
    let mut zk_client = TestCluster::new(
        &TestClusterOptions::new(APP_NAME, app_name),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
            pods_terminated_delay: Some(Duration::from_secs(5)),
        },
    );

    let zk_version = ZookeeperVersion::v3_5_8;

    let (zk_cr, zk_replicas) =
        build_zk_cluster(zk_client.name(), &zk_version, 1, Some(8080), None)?;
    zk_client.create_or_update(
        &zk_cr,
        &BTreeMap::from_iter([(String::from(APP_VERSION_LABEL), zk_version.to_string())]),
        zk_replicas,
    )?;

    Ok(zk_client)
}

fn build_zk_cluster(
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