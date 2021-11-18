pub mod common;

use anyhow::Result;
use common::kafka::{build_kafka_cluster, build_test_cluster};
use integration_test_commons::operator::checks::monitoring_checks;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::stackable_operator::k8s_openapi::api::core::v1::Pod;
use stackable_kafka_crd::APP_NAME;
use std::collections::BTreeMap;

#[test]
fn test_create_1_server_2_8_1() -> Result<()> {
    let version = "2.8.1";
    let mut cluster = build_test_cluster();

    let (kafka_cr, expected_pod_count) = build_kafka_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&kafka_cr, &BTreeMap::new(), expected_pod_count)
}
