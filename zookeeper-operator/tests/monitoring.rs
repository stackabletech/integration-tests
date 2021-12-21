pub mod common;

use crate::common::checks::custom_checks;

use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_cluster};
use integration_test_commons::operator::checks::monitoring_checks;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let replicas = 3;
    let container_name = "zookeeper";
    let version = "3.5.8";
    let admin_port = 8080;
    let metrics_port = 9505;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), version, replicas)?;

    cluster.create_or_update(&zookeeper_cr, &version_label(version), expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    let admin_service =
        create_node_port_service(&cluster.client, "zookeeper-admin", "zookeeper", admin_port);

    custom_checks(created_pods.as_slice(), version, &admin_service)?;

    // container names must to be lowercase
    let container_ports = vec![
        ("zk", 2181),
        ("zk-leader", 2888),
        ("zk-election", 3888),
        ("metrics", metrics_port),
    ];

    let metrics_service = create_node_port_service(
        &cluster.client,
        "zookeeper-metrics",
        "zookeeper",
        metrics_port,
    );

    monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
        &metrics_service,
    )?;

    Ok(())
}
