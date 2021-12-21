pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{build_test_cluster, build_zk_cluster};

use anyhow::Result;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let admin_port: i32 = 8080;
    let version = "3.5.8";

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 1)?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 3)?;

    cluster.create_or_update(
        &zookeeper_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;
    let created_pods = cluster.list::<Pod>(None);

    let admin_service =
        create_node_port_service(&cluster.client, "zookeeper-admin", "zookeeper", admin_port);

    custom_checks(created_pods.as_slice(), &version, &admin_service)?;

    Ok(())
}
