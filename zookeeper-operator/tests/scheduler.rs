pub mod common;

use anyhow::Result;
use common::zookeeper::{build_test_cluster, build_zk_cluster};
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

/// Tests that pods are rescheduled on the same nodes.
/// Requirements: At least 2 nodes.
/// How it works:
/// 1. creates a cluster with 1 replica and saves the selected node names.
/// 2. deletes all pods by creating a cluster with 0 replicas
/// 3. recreates the replica again and compares the selected node names with the ones from step 1.
#[test]
fn test_scheduler_reschedule_pods_on_the_same_nodes() -> Result<()> {
    let version = ZookeeperVersion::v3_5_8;
    let mut cluster = build_test_cluster();

    assert!(
        cluster.list_nodes(None).len() > 2,
        "Test requirements failed: at least two nodes!"
    );

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 1)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let node_names_1: Vec<String> = cluster
        .list::<Pod>(None)
        .iter()
        .map(|p| {
            p.spec
                .as_ref()
                .map_or("".to_string(), |s| s.node_name.as_ref().unwrap().clone())
        })
        .collect();

    // delete all pods
    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 0)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;

    assert_eq!(
        cluster.list::<Pod>(None).len(),
        0,
        "Test requirements failed: not all pods have been deleted!"
    );

    // recreate pods
    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 1)?;
    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let node_names_2: Vec<String> = cluster
        .list::<Pod>(None)
        .iter()
        .map(|p| {
            p.spec
                .as_ref()
                .map_or("".to_string(), |s| s.node_name.as_ref().unwrap().clone())
        })
        .collect();

    assert_eq!(
        node_names_1, node_names_2,
        "Pods rescheduled on the same nodes."
    );

    Ok(())
}
