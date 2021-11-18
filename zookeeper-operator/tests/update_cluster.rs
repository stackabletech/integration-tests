pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{build_test_cluster, build_zk_cluster, version_label};
use anyhow::Result;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::stackable_operator::kube::ResourceExt;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;
use std::thread;
use std::time::Duration;

#[test]
fn test_cluster_update() -> Result<()> {
    let replicas = 3;
    let admin_port: i32 = 9090;
    let client_port: i32 = 2182;
    let version = ZookeeperVersion::v3_5_8;
    let version_update = ZookeeperVersion::v3_7_0;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version,
        replicas,
        Some(admin_port),
        Some(client_port),
    )?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    cluster.check_pod_version(&version.to_string())?;

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version_update,
        replicas,
        Some(admin_port),
        Some(client_port),
    )?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version_update.to_string()),
        expected_pod_count,
    )?;

    let created_pods = cluster.list::<Pod>(None);

    let admin_service =
        create_node_port_service(&cluster.client, "zookeeper-admin", "zookeeper", admin_port);

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version_update,
        expected_pod_count,
        &admin_service,
    )?;

    cluster.check_pod_version(&version_update.to_string())?;

    Ok(())
}
