pub mod common;

use anyhow::Result;
use common::kafka::{build_kafka_cluster, build_test_cluster};
use integration_test_commons::operator::setup::version_label;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let version = "2.8.0";
    let mut cluster = build_test_cluster();

    let (kafka_cluster, expected_pod_count) = build_kafka_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(
        &kafka_cluster,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let (kafka_cluster, expected_pod_count) = build_kafka_cluster(cluster.name(), version, 3)?;
    cluster.create_or_update(
        &kafka_cluster,
        &version_label(&version.to_string()),
        expected_pod_count,
    )
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let version = "2.8.0";
    let mut cluster = build_test_cluster();

    let (kafka_cluster, expected_pod_count) = build_kafka_cluster(cluster.name(), version, 3)?;
    cluster.create_or_update(
        &kafka_cluster,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let (kafka_cluster, expected_pod_count) = build_kafka_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(
        &kafka_cluster,
        &version_label(&version.to_string()),
        expected_pod_count,
    )
}
