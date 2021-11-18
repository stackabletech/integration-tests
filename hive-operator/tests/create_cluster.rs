pub mod common;

use anyhow::{anyhow, Result};
use common::hive::{build_hive_cluster, build_test_cluster};
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;
use std::process::Command;
use std::{thread, time};

#[test]
fn test_create_1_server_2_3_9() -> Result<()> {
    let version = "2.3.9";
    let mut cluster = build_test_cluster();

    let (hive_cr, expected_pod_count) = build_hive_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(
        &hive_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

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

    // Check if the metastore is running on the pod
    for pod in created_pods {
        // extract hostname from port
        let node_name = match &pod.spec.as_ref().unwrap().node_name {
            None => {
                return Err(anyhow!(
                "Missing node_name in pod [{}]. Cannot create host address for metrics port check!",
                pod.metadata.name.as_ref().unwrap(),
            ))
            }
            Some(name) => name,
        };

        println!("Running python healthcheck script ...");
        let status = Command::new("/integration-tests/hive-operator/python/test_metastore.py")
            .args(["-a", node_name])
            .status()
            .expect("Failed to execute healthcheck script.");

        assert!(status.success());
    }

    Ok(())
}
