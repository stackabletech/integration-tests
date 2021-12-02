pub mod common;

use anyhow::Result;
use common::kafka::{build_kafka_cluster, build_test_cluster};
use common::zookeeper::build_zk_test_cluster;
use std::collections::BTreeMap;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let version = "2.8.1";
    let mut cluster = build_test_cluster();

    let zk_client = build_zk_test_cluster("test-kafka-zk")?;

    let (kafka_cluster, expected_pod_count) =
        build_kafka_cluster(cluster.name(), version, 1, zk_client.name())?;
    cluster.create_or_update(&kafka_cluster, &BTreeMap::new(), expected_pod_count)?;

    let (kafka_cluster, expected_pod_count) =
        build_kafka_cluster(cluster.name(), version, 3, zk_client.name())?;
    cluster.create_or_update(&kafka_cluster, &BTreeMap::new(), expected_pod_count)
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let version = "2.8.1";
    let mut cluster = build_test_cluster();

    let zk_client = build_zk_test_cluster("test-kafka-zk")?;

    let (kafka_cluster, expected_pod_count) =
        build_kafka_cluster(cluster.name(), version, 3, zk_client.name())?;
    cluster.create_or_update(&kafka_cluster, &BTreeMap::new(), expected_pod_count)?;

    let (kafka_cluster, expected_pod_count) =
        build_kafka_cluster(cluster.name(), version, 1, zk_client.name())?;
    cluster.create_or_update(&kafka_cluster, &BTreeMap::new(), expected_pod_count)
}
