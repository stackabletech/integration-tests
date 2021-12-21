pub mod common;

use anyhow::{anyhow, Result};
use common::hive::{build_hive_cluster, build_test_cluster};
use integration_test_commons::operator::checks::wait_for_scan_port;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::Pod;
use stackable_hive_crd::APP_NAME;
use std::collections::BTreeMap;
use std::process::Command;
use std::time::Duration;

#[test]
fn test_create_1_server_2_3_9() -> Result<()> {
    let version = "2.3.9";
    let mut cluster = build_test_cluster();

    let (hive_cr, expected_pod_count) = build_hive_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&hive_cr, &BTreeMap::new(), expected_pod_count)?;

    let created_pods = cluster.list::<Pod>(None);
    let actual_pod_count = created_pods.len();

    if actual_pod_count != expected_pod_count {
        return Err(anyhow!(
            "Expected {} pods but got {}!",
            expected_pod_count,
            actual_pod_count
        ));
    }

    let admin_service = create_node_port_service(&cluster.client, "hive-admin", APP_NAME, 9083);
    let metrics_service = create_node_port_service(&cluster.client, "hive-metrics", APP_NAME, 9084);

    // Check if the metastore is running on the pod
    for pod in created_pods {
        let hive_address = admin_service.address(&pod);
        let metrics_address = metrics_service.address(&pod);

        wait_for_scan_port(&hive_address, Duration::from_secs(60))?;
        wait_for_scan_port(&metrics_address, Duration::from_secs(60))?;

        let split: Vec<_> = hive_address.split(':').collect();

        let ip = split.get(0).unwrap();
        let port = split.get(1).unwrap();

        println!(
            "Running python health check script for [{}] ...",
            hive_address
        );
        let status = Command::new("python/test_metastore.py")
            .args(["-a", ip])
            .args(["-p", port])
            .status()
            .expect("Failed to execute health check script.");

        assert!(status.success());
    }

    Ok(())
}
