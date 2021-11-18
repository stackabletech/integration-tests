pub mod common;

use crate::common::checks::{custom_checks, custom_monitoring_checks};

use crate::common::zookeeper::version_label;
use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_cluster_with_metrics};
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let replicas = 3;
    let container_name = "zookeeper";
    let admin_port: i32 = 8082;
    let client_port: i32 = 2183;
    let metrics_port: i32 = 9506;
    let version = ZookeeperVersion::v3_5_8;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster_with_metrics(
        cluster.name(),
        &version,
        replicas,
        Some(admin_port),
        Some(client_port),
        Some(metrics_port),
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

    // container names must to be lowercase
    let container_ports = vec![
        ("metrics", metrics_port),
        ("client", client_port),
        ("admin", admin_port),
    ];

    let metrics_service = create_node_port_service(
        &cluster.client,
        "zookeeper-metrics",
        "zookeeper",
        metrics_port,
    );

    custom_monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
        &metrics_service,
    )?;

    Ok(())
}
