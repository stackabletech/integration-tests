pub mod common;

use anyhow::Result;
use common::kafka::{build_kafka_cluster, build_test_cluster};
use common::zookeeper::build_zk_test_cluster;
use std::collections::BTreeMap;

#[test]
fn test_create_1_server_2_8_1() -> Result<()> {
    let version = "2.8.1";

    let zk_client = build_zk_test_cluster("test-kafka-zk")?;

    let mut cluster = build_test_cluster();
    let (kafka_cr, expected_pod_count) =
        build_kafka_cluster(cluster.name(), version, 1, zk_client.name())?;
    cluster.create_or_update(&kafka_cr, &BTreeMap::new(), expected_pod_count)
}
