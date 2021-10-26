pub mod common;

use crate::common::checks::{custom_checks, custom_monitoring_checks};

use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_cluster_with_metrics_and_client_port};
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = "zookeeper";
    let client_port = 2181;
    let metrics_port = 9505;
    let version = ZookeeperVersion::v3_5_8;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster_with_metrics_and_client_port(
        cluster.name(),
        &version,
        1,
        client_port,
        metrics_port,
    )?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version,
        8080,
        expected_pod_count,
    )?;

    // container names must to be lowercase
    let container_ports = vec![
        ("metrics", metrics_port),
        ("client", client_port),
        ("admin", 8080),
    ];

    custom_monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
    )?;

    Ok(())
}
