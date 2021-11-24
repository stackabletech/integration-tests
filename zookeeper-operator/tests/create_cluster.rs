pub mod common;

use crate::common::checks::custom_checks;

use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_cluster};
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_create_cluster_3_5_8() -> Result<()> {
    let replicas = 3;
    let admin_port: i32 = 8080;
    let version = ZookeeperVersion::v3_5_8;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) =
        build_zk_cluster(cluster.name(), &version, replicas, Some(admin_port), None)?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;
    let created_pods = cluster.list::<Pod>(None);

    let admin_service =
        create_node_port_service(&cluster.client, "zookeeper-admin", "zookeeper", admin_port);

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        expected_pod_count,
        &admin_service,
    )?;

    Ok(())
}

#[test]
fn test_create_cluster_3_7_0() -> Result<()> {
    let replicas = 3;
    let admin_port: i32 = 8080;
    let client_port: i32 = 2181;
    let version = ZookeeperVersion::v3_7_0;

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
    let created_pods = cluster.list::<Pod>(None);

    let admin_service =
        create_node_port_service(&cluster.client, "zookeeper-admin", "zookeeper", admin_port);

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        expected_pod_count,
        &admin_service,
    )?;

    Ok(())
}
