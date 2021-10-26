pub mod common;

use crate::common::checks::custom_monitoring_checks;

use anyhow::Result;
use common::kafka::{build_kafka_cluster_monitoring, build_test_cluster};
use integration_test_commons::test::prelude::Pod;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = stackable_kafka_crd::APP_NAME;
    let metrics_port: u16 = 9505;
    let version = "2.8.0";

    let mut cluster = build_test_cluster();

    let (kafka_cr, expected_pod_count) =
        build_kafka_cluster_monitoring(cluster.name(), version, 1, metrics_port)?;
    cluster.create_or_update(&kafka_cr, expected_pod_count)?;

    let created_pods = cluster.list::<Pod>(None);
    // container names need to be lowercase
    let container_ports = vec![("metrics", metrics_port)];

    custom_monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
    )?;

    Ok(())
}
