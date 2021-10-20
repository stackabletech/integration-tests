pub mod common;

use anyhow::Result;
use common::monitoring::{build_monitoring_cluster, build_test_cluster};

#[test]
fn test_create_cluster() -> Result<()> {
    let version = "2.28.1";
    let aggregator_port: u16 = 9101;
    let node_exporter_port: u16 = 9102;
    let federation_port: u16 = 9103;

    let mut cluster = build_test_cluster();
    let available_nodes = cluster.list_nodes(None).len();

    let (monitoring_cr, expected_pod_count) = build_monitoring_cluster(
        cluster.name(),
        version,
        available_nodes,
        aggregator_port,
        node_exporter_port,
        federation_port,
    )?;

    cluster.create_or_update(&monitoring_cr, expected_pod_count)?;

    Ok(())
}
