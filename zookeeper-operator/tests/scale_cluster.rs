pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{build_test_cluster, build_zk_cluster};

use anyhow::Result;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

// This will cause the integration tests to fail because config maps are not updated correctly. This
// can be activated once https://github.com/stackabletech/zookeeper-operator/issues/128 is fixed.
#[test]
#[ignore]
fn test_scale_cluster_up() -> Result<()> {
    let admin_port: i32 = 8080;
    let client_port: i32 = 2181;
    let version = ZookeeperVersion::v3_5_8;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version,
        1,
        Some(admin_port),
        Some(client_port),
    )?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version,
        3,
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

// This will cause the integration tests to fail because config maps are not updated correctly. This
// can be activated once https://github.com/stackabletech/zookeeper-operator/issues/128 is fixed.
#[test]
#[ignore]
fn test_scale_cluster_down() -> Result<()> {
    let version = ZookeeperVersion::v3_5_8;
    let admin_port: i32 = 8080;
    let client_port: i32 = 2181;
    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version,
        3,
        Some(admin_port),
        Some(client_port),
    )?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version,
        1,
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
