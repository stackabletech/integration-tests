pub mod common;

use anyhow::{anyhow, Result};
use common::hive::{build_hive_cluster, build_test_cluster};
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::Pod;
use stackable_hive_crd::APP_NAME;
use std::collections::BTreeMap;
use std::process::Command;
use std::{thread, time};

#[test]
fn test_create_1_server_2_3_9() -> Result<()> {
    let version = "2.3.9";
    let mut cluster = build_test_cluster();

    let (hive_cr, expected_pod_count) = build_hive_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&hive_cr, &BTreeMap::new(), expected_pod_count)?;

    // TODO: remove! This should be checked via open ports or other readiness probe.
    // Wait for the metastore to have started fully
    let delay_time = time::Duration::from_secs(40);
    thread::sleep(delay_time);

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

    // Check if the metastore is running on the pod
    for pod in created_pods {
        let address = admin_service.address(&pod);
        let split: Vec<_> = address.split(':').collect();

        let ip = split.get(0).unwrap();
        let port = split.get(1).unwrap();

        println!("Running python health check script for [{}] ...", address);
        let status = Command::new("python/test_metastore.py")
            .args(["-a", ip])
            .args(["-p", port])
            .status()
            .expect("Failed to execute health check script.");

        assert!(status.success());
    }

    Ok(())
}
