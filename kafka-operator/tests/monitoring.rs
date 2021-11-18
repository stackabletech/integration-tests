pub mod common;

use anyhow::Result;
use common::kafka::{build_kafka_cluster_monitoring, build_test_cluster};
use integration_test_commons::operator::checks::monitoring_checks;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::Pod;
use std::collections::BTreeMap;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = stackable_kafka_crd::APP_NAME;
    let metrics_port: i32 = 9707;
    let version = "2.8.1";

    let mut cluster = build_test_cluster();

    let (kafka_cr, expected_pod_count) =
        build_kafka_cluster_monitoring(cluster.name(), version, 1, metrics_port)?;
    cluster.create_or_update(&kafka_cr, &BTreeMap::new(), expected_pod_count)?;

    let metric_service = create_node_port_service(
        &cluster.client,
        "kafka-metric",
        stackable_kafka_crd::APP_NAME,
        metrics_port,
    );
    let created_pods = cluster.list::<Pod>(None);

    // container names need to be lowercase
    let container_ports = vec![("metrics", metrics_port)];

    monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
        &metric_service,
    )
}
