pub mod common;

use anyhow::Result;
use common::kafka::{build_kafka_cluster, build_test_cluster};
use integration_test_commons::operator::setup::version_label;

#[test]
fn test_create_1_server_2_8_0() -> Result<()> {
    let version = "2.8.1";
    let mut cluster = build_test_cluster();

    let (kafka_cr, expected_pod_count) = build_kafka_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(
        &kafka_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )
}
