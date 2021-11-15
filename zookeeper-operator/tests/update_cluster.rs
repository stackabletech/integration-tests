pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{build_test_cluster, build_zk_cluster};
use anyhow::Result;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;
use std::thread;
use std::time::Duration;

#[test]
fn test_cluster_update() -> Result<()> {
    let version = ZookeeperVersion::v3_5_8;
    let version_update = ZookeeperVersion::v3_7_0;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 1)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        2181,
        expected_pod_count,
    )?;
    check_pod_version(&version, created_pods.as_slice(), &cluster.labels.version);

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version_update, 1)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version_update,
        8080,
        expected_pod_count,
    )?;
    check_pod_version(
        &version_update,
        created_pods.as_slice(),
        &cluster.labels.version,
    );

    thread::sleep(Duration::from_secs(2));

    Ok(())
}

fn check_pod_version(version: &ZookeeperVersion, pods: &[Pod], version_label: &str) {
    for pod in pods {
        let pod_version = pod
            .metadata
            .labels
            .as_ref()
            .and_then(|labels| labels.get(version_label).cloned());
        assert_eq!(Some(version.to_string()), pod_version);
    }
}
