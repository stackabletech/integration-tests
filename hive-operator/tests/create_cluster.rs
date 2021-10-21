pub mod common;

use anyhow::{anyhow, Result};
use common::hive::{build_hive_cluster, build_test_cluster};
use integration_test_commons::test::prelude::Pod;
use std::process::Command;
use std::{thread, time};
use std::fs;

const DB_PATH: &str = "/tmp/metadata_db";

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {

        println!("Removing Derby DB ...");
        fs::remove_dir_all(DB_PATH);
        // TODO doesn't work because we're not in the right container

    }
}

#[test]
fn test_create_1_server_2_3_9() -> Result<()> {
    let cleanup = Cleanup;
    
    let version = "2.3.9";
    let mut cluster = build_test_cluster();

    let (hive_cr, expected_pod_count) = build_hive_cluster(cluster.name(), version, DB_PATH, 1)?;
    cluster.create_or_update(&hive_cr, expected_pod_count)?;

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
            None => return Err(anyhow!(
                "Missing node_name in pod [{}]. Cannot create host address for metrics port check!",
                pod.metadata.name.as_ref().unwrap(),
            )),
            Some(name) => name,
        };

        println!("Running python script");
        let status = Command::new("/integration-tests/hive-operator/python/test_metastore.py")
            .args(["-a", node_name])
            .status()
            .expect("Failed to execute python script.");

        assert!(status.success());
    }

    Ok(())
}
